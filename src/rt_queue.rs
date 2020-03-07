use alloc::alloc::{alloc, dealloc};
use alloc::sync::Arc;
use core::alloc::Layout;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

/// A wait-free, single producer, single consumer, thread safe ring buffer. Notably, elements do
/// not need to implement `Copy`. The ring size must be a power of 2 and `new` will panic if it
/// isn't.
///
/// Probably unsound/buggy. Don't use it.
pub struct AtomicRing<T: Send> {
    buf: *mut MaybeUninit<T>,
    length: usize,
    read_ptr: AtomicUsize,
    write_ptr: AtomicUsize,
}

unsafe impl<T: Send> Send for AtomicRing<T> {}
unsafe impl<T: Send> Sync for AtomicRing<T> {}

impl<T: Send> AtomicRing<T> {
    pub fn new(length: usize) -> (AtomicRingReader<T>, AtomicRingWriter<T>) {
        assert!(length > 0 && length.is_power_of_two());

        let state = Arc::new(Self {
            buf: unsafe { alloc(Layout::array::<MaybeUninit<T>>(length).unwrap()) }
                as *mut MaybeUninit<T>,
            length: length,
            read_ptr: AtomicUsize::new(0),
            write_ptr: AtomicUsize::new(0),
        });

        (AtomicRingReader(state.clone()), AtomicRingWriter(state))
    }
}

impl<T: Send> Drop for AtomicRing<T> {
    fn drop(&mut self) {
        let mut read_ptr = self.read_ptr.load(Ordering::SeqCst);
        let write_ptr = self.write_ptr.load(Ordering::SeqCst);

        while read_ptr < write_ptr {
            let read_masked = read_ptr & (self.length - 1);

            unsafe {
                drop((*self.buf.offset(read_masked as isize)).read());
            }

            read_ptr += 1;
        }

        unsafe {
            dealloc(
                self.buf as *mut u8,
                Layout::array::<MaybeUninit<T>>(self.length).unwrap(),
            )
        }
    }
}

/// The read side of an atomic ring buffer.
pub struct AtomicRingReader<T: Send>(Arc<AtomicRing<T>>);
/// The write side of an atomic ring buffer.
pub struct AtomicRingWriter<T: Send>(Arc<AtomicRing<T>>);

impl<T: Send> AtomicRingReader<T> {
    /// At least this many items can be read from the buffer.
    pub fn read_available(&self) -> usize {
        let write_ptr = self.0.write_ptr.load(Ordering::SeqCst);
        let read_ptr = self.0.read_ptr.load(Ordering::SeqCst);

        write_ptr - read_ptr
    }

    /// Tries to remove an item from the buffer, returning `None` if no item could be read.
    pub fn try_pop(&mut self) -> Option<T> {
        let read_ptr = self.0.read_ptr.load(Ordering::SeqCst);
        let write_ptr = self.0.write_ptr.load(Ordering::SeqCst);
        let read_masked = read_ptr & (self.0.length - 1);

        if write_ptr == read_ptr {
            return None;
        }

        let result = unsafe { (*self.0.buf.offset(read_masked as isize)).read() };
        self.0
            .read_ptr
            .store(read_ptr.wrapping_add(1), Ordering::SeqCst);
        Some(result)
    }
}

impl<T: Send> AtomicRingWriter<T> {
    pub fn write_available(&self) -> usize {
        let read_ptr = self.0.read_ptr.load(Ordering::SeqCst);
        let write_ptr = self.0.write_ptr.load(Ordering::SeqCst);

        self.0.length - (write_ptr - read_ptr)
    }

    /// Tries to insert the item into the buffer. If successful, returns `None`, otherwise returns
    /// `Some(value)`.
    pub fn try_push(&mut self, value: T) -> Option<T> {
        let read_ptr = self.0.read_ptr.load(Ordering::SeqCst);
        let write_ptr = self.0.write_ptr.load(Ordering::SeqCst);
        let write_masked = write_ptr & (self.0.length - 1);

        if write_ptr - read_ptr == self.0.length {
            return Some(value);
        }

        unsafe { (*self.0.buf.offset(write_masked as isize)).write(value) };
        self.0
            .write_ptr
            .store(write_ptr.wrapping_add(1), Ordering::SeqCst);
        None
    }

    /// Tries to copy a slice of values into the buffer. Returns the number of values successfully
    /// copied from the slice.
    pub fn try_pushn<Q>(&mut self, value: &[Q]) -> usize
    where
        Q: Clone + Into<T>,
    {
        // TODO(eiz): efficient implementation of this.
        let mut copied = 0;

        for item in value {
            if let Some(_) = self.try_push(item.clone().into()) {
                break;
            }

            copied += 1;
        }

        copied
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::{AtomicBool, AtomicU64};
    use std::thread;

    #[test]
    pub fn works_single_thread() {
        let (mut read, mut write) = AtomicRing::new(2);

        assert!(write.try_push("hello").is_none());
        assert!(write.try_push("world").is_none());
        assert!(read.try_pop().unwrap() == "hello");
        assert!(write.try_push("three").is_none());
        assert!(read.try_pop().unwrap() == "world");
        assert!(read.try_pop().unwrap() == "three");
    }

    #[derive(Default)]
    struct LeakCounter {
        allocated: AtomicU64,
        dropped: AtomicU64,
    }

    impl LeakCounter {
        pub fn allocated(&self) -> u64 {
            self.allocated.load(Ordering::SeqCst)
        }

        pub fn dropped(&self) -> u64 {
            self.dropped.load(Ordering::SeqCst)
        }

        pub fn alive(&self) -> u64 {
            self.allocated() - self.dropped()
        }
    }

    struct LeakProbe<T> {
        counter: Arc<LeakCounter>,
        inner: T,
    }

    impl<T> LeakProbe<T> {
        pub fn new(inner: T, counter: Arc<LeakCounter>) -> Self {
            counter.allocated.fetch_add(1, Ordering::SeqCst);
            Self { counter, inner }
        }

        pub fn inner(&self) -> &T {
            &self.inner
        }
    }

    impl<T> Drop for LeakProbe<T> {
        fn drop(&mut self) {
            self.counter.dropped.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    pub fn multi_thread_doesnt_blatantly_crash() {
        let counter = Arc::new(LeakCounter::default());
        let counter_r = counter.clone();
        let (mut read, mut write) = AtomicRing::new(1024);
        let should_exit = Arc::new(AtomicBool::new(false));
        let should_exit_r = should_exit.clone();
        let join_w = thread::spawn(move || {
            let mut total_written = 0;

            for i in 0..100 {
                for j in 0..1024 {
                    if write
                        .try_push(LeakProbe::new(
                            (format!("{}", i + 1), format!("{}", j + 1)),
                            counter.clone(),
                        ))
                        .is_none()
                    {
                        total_written += 1;
                    }
                }
            }

            assert!(total_written >= 1024);
            should_exit.store(true, Ordering::Relaxed);
        });
        let join_r = thread::spawn(move || {
            let mut total_read = 0;
            let mut last_i = 0;
            let mut last_j = 0;

            while total_read == 0 || should_exit_r.load(Ordering::Relaxed) == false {
                for _ in 0..1024 {
                    if let Some(item) = read.try_pop() {
                        let (i_str, j_str) = item.inner();
                        total_read += 1;
                        let new_i = i_str.parse::<u32>().unwrap();
                        let new_j = j_str.parse::<u32>().unwrap();

                        assert!(new_i > last_i || new_j > last_j);
                        last_i = new_i;
                        last_j = new_j;
                    }
                }
            }

            assert!(total_read >= 1024);
        });

        join_r.join().unwrap();
        join_w.join().unwrap();
        assert_eq!(counter_r.allocated(), 100 * 1024);
        assert_eq!(counter_r.alive(), 0);
    }
}
