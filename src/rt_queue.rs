//! Provides queues suitable for use in real-time programming. Real-time in this sense is defined
//! as "threads which need to service events on a fixed deadline." For example, audio or other
//! hardware interfacing where the hardware will produce incorrect behavior if a deadline is
//! missed. Special care is taken to provide interfaces which are not subject to priority inversion
//! or other thread scheduling anomalies, to the extent possible.
//!
//! Probably unsound/buggy. Don't use it. Have you tried crossbeam channels?
use alloc::alloc::{alloc, dealloc};
use alloc::sync::Arc;
use core::alloc::Layout;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

/// The `try_push` operation.
pub trait Push<T: Send> {
    /// Tries to insert `value` into the container. If successful, returns `None`, otherwise
    /// returns `Some(value)`.
    fn try_push(&mut self, value: T) -> Option<T>;
}

/// The `push` operation.
pub trait PushWait<T: Send> {
    /// Inserts `value` into the container, waiting for available space if needed.
    fn push(&mut self, value: T);
}

/// The `try_pushn` operation.
pub trait PushN<T: Send + Clone> {
    /// Tries to copy a slice of values into the container. Returns the number of values
    /// successfully copied from the slice.
    fn try_pushn(&mut self, value: &[T]) -> usize;
}

/// The `pushn` operation.
pub trait PushNWait<T: Send + Clone> {
    /// Pushes multiple items from `value` into the container, waiting for available space if
    /// needed.
    fn pushn(&mut self, value: &[T]);
}

/// The `try_pushn_copy` operation.
pub trait PushNCopy<T: Send + Copy> {
    fn try_pushn_copy(&mut self, value: &[T]) -> usize;
}

/// The `pushn_copy` operation.
pub trait PushNCopyWait<T: Send + Copy> {
    fn pushn_copy(&mut self, value: &[T]);
}

/// The `try_pop` operation.
pub trait Pop<T: Send> {
    /// Tries to remove an item from the container, returning `None` if no item could be read.
    fn try_pop(&mut self) -> Option<T>;
}

/// The `pop` operation.
pub trait PopWait<T: Send> {
    /// Removes an item from the container, waiting for a writer if needed.
    fn pop(&mut self) -> T;
}

/// The `try_popn` operation.
pub trait PopN<T: Send> {
    fn try_popn(&mut self, dst: &mut [T]) -> usize;
}

/// The `try_popn_copy` operation.
pub trait PopNCopy<T: Send + Copy> {
    fn try_popn_copy(&mut self, dst: &mut [T]) -> usize;
}

/// A wait-free, single producer, single consumer, thread safe ring buffer.
///
/// Notably, elements do not need to implement `Copy`. The ring size must be a power of 2 and `new`
/// will panic if it isn't. Merely lock-free if atomic load/store aren't wait-free.
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
        assert!(length.is_power_of_two());

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
        let read_ptr = self.0.read_ptr.load(Ordering::Acquire);
        let write_ptr = self.0.write_ptr.load(Ordering::Acquire);

        write_ptr - read_ptr
    }
}

impl<T: Send> Pop<T> for AtomicRingReader<T> {
    fn try_pop(&mut self) -> Option<T> {
        let read_ptr = self.0.read_ptr.load(Ordering::Acquire);
        let write_ptr = self.0.write_ptr.load(Ordering::Acquire);
        let read_masked = read_ptr & (self.0.length - 1);

        if write_ptr == read_ptr {
            return None;
        }

        let result = unsafe { (*self.0.buf.offset(read_masked as isize)).read() };
        self.0
            .read_ptr
            .store(read_ptr.wrapping_add(1), Ordering::Release);
        Some(result)
    }
}

impl<T: Send> AtomicRingWriter<T> {
    pub fn write_available(&self) -> usize {
        let read_ptr = self.0.read_ptr.load(Ordering::Acquire);
        let write_ptr = self.0.write_ptr.load(Ordering::Acquire);

        self.0.length - (write_ptr - read_ptr)
    }
}

impl<T: Send> Push<T> for AtomicRingWriter<T> {
    fn try_push(&mut self, value: T) -> Option<T> {
        let read_ptr = self.0.read_ptr.load(Ordering::Acquire);
        let write_ptr = self.0.write_ptr.load(Ordering::Acquire);
        let write_masked = write_ptr & (self.0.length - 1);

        if write_ptr - read_ptr == self.0.length {
            return Some(value);
        }

        unsafe { (*self.0.buf.offset(write_masked as isize)).write(value) };
        self.0
            .write_ptr
            .store(write_ptr.wrapping_add(1), Ordering::Release);
        None
    }
}

impl<T: Send + Clone> PushN<T> for AtomicRingWriter<T> {
    fn try_pushn(&mut self, value: &[T]) -> usize {
        let mut copied = 0;
        let read_ptr = self.0.read_ptr.load(Ordering::Acquire);
        let write_ptr = self.0.write_ptr.load(Ordering::Acquire);
        let to_write = self.0.length - (write_ptr - read_ptr);

        for item in value {
            if copied == to_write {
                break;
            }

            let write_masked = (write_ptr + copied) & (self.0.length - 1);

            unsafe { (*self.0.buf.offset(write_masked as isize)).write(item.clone()) };
            copied += 1;
        }

        self.0
            .write_ptr
            .store(write_ptr.wrapping_add(copied), Ordering::Release);
        copied
    }
}

#[cfg(feature = "rt_queue_std")]
mod std_detail {
    use super::*;
    use parking_lot::{Condvar, Mutex};

    /// A simple bounded multiple producer, single consumer queue.
    ///
    /// The consumer side has a wait-free `try_pop` that always succeeds if there is data in the
    /// buffer. Synchronization is otherwise via a `parking_lot` mutex and condition variable.
    pub struct MpscQueue<T: Send> {
        ring_writer: Mutex<AtomicRingWriter<T>>,
        cond_read: Condvar,
    }

    impl<T: Send> MpscQueue<T> {
        /// Returns a new queue with a bound of at least `length`. In practice, it's rounded to the
        /// next power of two.
        pub fn new(length: usize) -> (MpscQueueReader<T>, MpscQueueWriter<T>) {
            let length = length.next_power_of_two();
            let (read, write) = AtomicRing::new(length);
            let instance = Arc::new(Self {
                ring_writer: Mutex::new(write),
                cond_read: Condvar::new(),
            });

            (
                MpscQueueReader {
                    inner: instance.clone(),
                    read,
                },
                MpscQueueWriter(instance),
            )
        }
    }

    /// The write side of an MPSC queue.
    #[derive(Clone)]
    pub struct MpscQueueWriter<T: Send>(Arc<MpscQueue<T>>);

    impl<T: Send> Push<T> for MpscQueueWriter<T> {
        fn try_push(&mut self, value: T) -> Option<T> {
            let mut ring_writer = self.0.ring_writer.lock();
            let result = ring_writer.try_push(value);

            if result.is_none() {
                self.0.cond_read.notify_one();
            }

            result
        }
    }

    impl<T: Send + Clone> PushN<T> for MpscQueueWriter<T> {
        fn try_pushn(&mut self, value: &[T]) -> usize {
            let mut ring_writer = self.0.ring_writer.lock();
            let result = ring_writer.try_pushn(value);

            if result > 0 {
                self.0.cond_read.notify_one();
            }

            result
        }
    }

    /// The read side an of an MPSC queue.
    pub struct MpscQueueReader<T: Send> {
        inner: Arc<MpscQueue<T>>,
        read: AtomicRingReader<T>,
    }

    impl<T: Send> Pop<T> for MpscQueueReader<T> {
        fn try_pop(&mut self) -> Option<T> {
            let result = self.read.try_pop();
            result
        }
    }

    impl<T: Send> PopWait<T> for MpscQueueReader<T> {
        fn pop(&mut self) -> T {
            let mut result = self.try_pop();

            if result.is_some() {
                return result.unwrap();
            }

            let mut ring_writer = self.inner.ring_writer.lock();

            loop {
                result = self.read.try_pop();

                if result.is_some() {
                    break;
                }

                self.inner.cond_read.wait(&mut ring_writer);
            }

            result.unwrap()
        }
    }
}

#[cfg(feature = "rt_queue_std")]
pub use std_detail::*;

#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::{AtomicBool, AtomicU64};
    use std::thread;
    use std::time::{Duration, Instant};

    /* it doesn't. TODO(eiz): re-implement self-cargo-invocation gunk.
    struct YouCantCloneMe;

    #[test]
    pub fn this_shouldnt_compile() {
        let (_, mut write) = AtomicRing::new(1);

        write.try_pushn(&[YouCantCloneMe]);
    }
     */

    #[test]
    pub fn works_single_thread() {
        let (mut read, mut write) = AtomicRing::new(2);

        assert!(write.try_push("hello".to_owned()).is_none());
        assert!(write.try_push("world".to_owned()).is_none());
        assert!(read.try_pop().unwrap() == "hello".to_owned());
        assert!(write.try_push("three".to_owned()).is_none());
        assert!(read.try_pop().unwrap() == "world");
        assert!(read.try_pop().unwrap() == "three");
        write.try_pushn(&["hello".to_owned(), "world".to_owned()]);
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

    #[test]
    pub fn stress_it() {
        let mut joins = vec![];

        for _ in 0..32 {
            joins.push(thread::spawn(|| {
                for _ in 0..100 {
                    multi_thread_doesnt_blatantly_crash();
                }
            }));
        }

        for j in joins.into_iter() {
            j.join().unwrap();
        }
    }

    #[test]
    pub fn write_multi_kinda_functions() {
        let (mut read, mut write) = AtomicRing::new(8);

        assert_eq!(write.try_pushn(&[1u8, 2, 3, 4, 5, 6, 7, 8, 9]), 8);

        for i in 0..8 {
            assert_eq!(read.try_pop(), Some(i + 1));
        }

        assert_eq!(write.try_pushn(&[1u8, 2, 3, 4, 5, 6, 7, 8, 9]), 8);
        assert_eq!(write.try_pushn(&[1u8, 2, 3, 4, 5, 6, 7, 8, 9]), 0);

        // force a wrap
        let (mut read, mut write) = AtomicRing::new(8);

        for _ in 0..2 {
            assert_eq!(write.try_pushn(&[1u8, 2, 3, 4, 5, 6]), 6);

            for i in 0..6 {
                assert_eq!(read.try_pop(), Some(i + 1));
            }
        }
    }

    #[test]
    pub fn mpsc_single_thread_two_writers() {
        #[derive(Clone, Debug, PartialEq, Eq)]
        enum Message {
            MessageA,
            MessageB,
        }

        let (mut read, write) = MpscQueue::new(2);
        let mut writer_a = write.clone();
        let mut writer_b = write.clone();

        assert!(writer_a.try_push(Message::MessageA).is_none());
        assert!(writer_b.try_push(Message::MessageB).is_none());
        assert_eq!(read.try_pop(), Some(Message::MessageA));
        assert_eq!(read.try_pop(), Some(Message::MessageB));
    }

    #[test]
    pub fn mpsc_pop_wait_waits() {
        let (mut read, mut write) = MpscQueue::new(1);
        let (mut started_read, mut started_write) = MpscQueue::new(1);
        let (mut finished_read, mut finished_write) = MpscQueue::new(1);
        let join_a = thread::spawn(move || {
            let start = Instant::now();

            assert!(started_write.try_push(()).is_none());
            assert_eq!(read.pop(), 1234);
            assert!(finished_write
                .try_push(Instant::now().duration_since(start))
                .is_none());
        });
        started_read.pop();
        thread::sleep(Duration::from_secs(2));
        write.try_push(1234);
        let elapsed = finished_read.pop();

        assert!(elapsed > Duration::from_secs(1));
        join_a.join().unwrap();
    }

    /*
     * This is currently not implemented because the semantics of `parking_lot::Condvar` don't
     * allow it to be implemented in a way that can't force the reader to wait on writers safely.
     * Need some other sync primitive first.
     *
    #[test]
    pub fn mpsc_push_wait_waits() {
        let (mut read, mut write) = MpscQueue::<Instant>::new(1);
        let (mut init_read, mut init_write) = MpscQueue::new(1);
        let join_a = thread::spawn(move || {
            let duration = init_read.pop();

            thread::sleep(duration);
            read.pop();
            let first = read.pop();
            let second = read.pop();

            assert!(second.duration_since(first) > duration / 2);
        });

        write.push(Instant::now());
        init_write.push(Duration::from_secs(1));
        write.push(Instant::now());
        write.push(Instant::now());
        join_a.join().unwrap();
    }
    */
}
