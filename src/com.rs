use core::borrow::Borrow;
use core::fmt;
use core::ops::Deref;
use core::ptr::{self, NonNull};
use std::marker::PhantomData;
use std::ops::DerefMut;
use winapi::shared::minwindef::UINT;
use winapi::shared::winerror::SUCCEEDED;
use winapi::shared::winerror::S_OK;
use winapi::shared::wtypes::BSTR;
use winapi::shared::wtypesbase::CLSCTX_INPROC_SERVER;
use winapi::um::combaseapi::CoCreateInstance;
use winapi::um::combaseapi::{CoInitializeEx, CoUninitialize};
use winapi::um::objbase::{COINIT_APARTMENTTHREADED, COINIT_MULTITHREADED};
use winapi::um::oleauto::SysAllocStringLen;
use winapi::um::oleauto::SysFreeString;
use winapi::um::oleauto::SysStringLen;
use winapi::um::unknwnbase::IUnknown;
use winapi::um::winnt::HRESULT;
use winapi::Class;
use winapi::Interface;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ComError(pub i32);

impl fmt::Debug for ComError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write![f, "0x{:08X}", self.0]
    }
}

impl fmt::Display for ComError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(feature = "use_std")]
impl std::error::Error for ComError {}

#[repr(transparent)]
pub struct ComPtr<T: Interface>(NonNull<T>);

impl<T: Interface> ComPtr<T> {
    pub fn create_inproc<C: Class>() -> Result<Self, ComError> {
        com_new(|x: *mut *mut T| unsafe {
            CoCreateInstance(
                &C::uuidof(),
                ptr::null_mut(),
                CLSCTX_INPROC_SERVER,
                &T::uuidof(),
                x as *mut _,
            )
        })
    }

    pub unsafe fn from_raw_unchecked(ptr: *mut T) -> Self {
        Self(NonNull::new_unchecked(ptr))
    }

    pub fn query_interface<Q: Interface>(&self) -> Result<ComPtr<Q>, ComError> {
        unsafe {
            com_new(|x| {
                (self.0.cast::<IUnknown>().as_ref()).QueryInterface(&Q::uuidof(), x as *mut *mut _)
            })
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    pub fn iter<V: Interface, F: Fn(&ComPtr<T>, *mut *mut V) -> HRESULT>(
        &self,
        f: F,
    ) -> impl core::iter::Iterator<Item = ComPtr<V>> {
        let outer = self.clone();

        ComIterator {
            f: move |x| (f)(&outer, x),
            _phantom: PhantomData,
        }
    }
}

impl<T: Interface> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        unsafe {
            (self.0.cast::<IUnknown>().as_ref()).AddRef();
        }

        Self(self.0)
    }
}

impl<T: Interface> fmt::Debug for ComPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write![f, "0x{:08X}", self.0.as_ptr() as usize]
    }
}

impl<T: Interface> Deref for ComPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T: Interface> Drop for ComPtr<T> {
    fn drop(&mut self) {
        unsafe {
            (self.0.cast::<IUnknown>().as_ref()).Release();
        }
    }
}

pub fn com_new<R, F>(f: F) -> Result<ComPtr<R>, ComError>
where
    F: FnOnce(*mut *mut R) -> HRESULT,
    R: Interface,
{
    let mut ptr: *mut R = ptr::null_mut();
    let hr = (f)(&mut ptr);

    if SUCCEEDED(hr) {
        if ptr.is_null() {
            panic!["invariant: new com object must not be null."];
        } else {
            Ok(unsafe { ComPtr::from_raw_unchecked(ptr) })
        }
    } else {
        Err(ComError(hr))
    }
}

pub fn com_new_bstr<F>(f: F) -> Result<BString, ComError>
where
    F: FnOnce(*mut BSTR) -> HRESULT,
{
    let mut ptr: BSTR = ptr::null_mut();
    let hr = (f)(&mut ptr);

    if SUCCEEDED(hr) {
        if ptr.is_null() {
            panic!["invariant: new BSTR must not be null."];
        } else {
            Ok(unsafe { BString::from_raw_unchecked(ptr) })
        }
    } else {
        Err(ComError(hr))
    }
}

pub fn com_new_void<R, F>(f: F) -> Option<ComPtr<R>>
where
    F: FnOnce(*mut *mut R),
    R: Interface,
{
    let mut ptr: *mut R = ptr::null_mut();
    (f)(&mut ptr);

    if ptr.is_null() {
        None
    } else {
        Some(unsafe { ComPtr::from_raw_unchecked(ptr) })
    }
}

pub fn com_hr<F>(f: F) -> Result<(), ComError>
where
    F: FnOnce() -> HRESULT,
{
    let hr = (f)();

    if SUCCEEDED(hr) {
        Ok(())
    } else {
        Err(ComError(hr))
    }
}

struct ComIterator<V: Interface, F: Fn(*mut *mut V) -> HRESULT> {
    f: F,
    _phantom: PhantomData<V>,
}

impl<V: Interface, F: Fn(*mut *mut V) -> HRESULT> core::iter::Iterator for ComIterator<V, F> {
    type Item = ComPtr<V>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ptr: *mut V = ptr::null_mut();

        match (self.f)(&mut ptr) {
            S_OK => {
                if ptr.is_null() {
                    panic!["invariant: COM iterator returned S_OK with no value"];
                } else {
                    Some(unsafe { ComPtr::from_raw_unchecked(ptr) })
                }
            }
            _ => None,
        }
    }
}

#[allow(dead_code)]
pub enum ComApartment {
    SingleThreaded,
    MultiThreaded,
}

pub struct ScopedComInitialize;

impl ScopedComInitialize {
    pub fn new(apartment: ComApartment) -> Self {
        let flags = match apartment {
            ComApartment::SingleThreaded => COINIT_APARTMENTTHREADED,
            ComApartment::MultiThreaded => COINIT_MULTITHREADED,
        };

        unsafe {
            com_hr(|| CoInitializeEx(ptr::null_mut(), flags)).expect("CoInitializeEx failed");
        }

        Self
    }
}

impl Drop for ScopedComInitialize {
    fn drop(&mut self) {
        unsafe { CoUninitialize() }
    }
}

#[repr(transparent)]
pub struct BStr(BSTR);

impl BStr {
    pub unsafe fn from_raw_unchecked(ptr: BSTR) -> Self {
        Self(ptr)
    }

    pub fn len(&self) -> UINT {
        unsafe { SysStringLen(self.0) }
    }

    pub fn as_ptr(&self) -> BSTR {
        self.0
    }

    #[cfg(feature = "use_std")]
    pub fn to_os_string(&self) -> std::ffi::OsString {
        unsafe {
            std::os::windows::ffi::OsStringExt::from_wide(std::slice::from_raw_parts(
                self.as_ptr(),
                self.len() as usize,
            ))
        }
    }
}

#[repr(transparent)]
pub struct BString(BSTR);

impl BString {
    pub unsafe fn from_raw_unchecked(ptr: BSTR) -> Self {
        Self(ptr)
    }

    pub fn from_slice(chars: &[u16]) -> Self {
        assert!(chars.len() <= u32::MAX as usize);
        Self(unsafe { SysAllocStringLen(chars.as_ptr(), chars.len() as u32) })
    }
}

impl Deref for BString {
    type Target = BStr;

    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

impl DerefMut for BString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}

impl Borrow<BStr> for BString {
    fn borrow(&self) -> &BStr {
        unsafe { core::mem::transmute(self) }
    }
}

impl Drop for BString {
    fn drop(&mut self) {
        unsafe { SysFreeString(self.0) }
    }
}
