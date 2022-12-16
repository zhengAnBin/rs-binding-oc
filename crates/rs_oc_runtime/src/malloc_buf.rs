extern crate libc;
use core::slice;
use libc::{c_char, c_void};
use std::{ops::Deref, ptr, str};
const DUMMY_PTR: *mut c_void = 0x1 as *mut c_void;

pub struct Malloc<T: ?Sized> {
    ptr: *mut T,
}

impl<T> Malloc<T> {
    pub unsafe fn from_ptr(ptr: *mut T) -> Malloc<T> {
        Malloc { ptr }
    }

    pub unsafe fn from_array(ptr: *mut T, len: usize) -> Malloc<[T]> {
        let ptr = if ptr.is_null() && len == 0 {
            DUMMY_PTR as *mut T
        } else {
            ptr
        };
        let slice = slice::from_raw_parts_mut(ptr, len);
        Malloc {
            ptr: slice as *const [T] as *mut [T],
        }
    }
}

impl Malloc<str> {
    pub unsafe fn from_c_str(ptr: *mut c_char) -> Result<Malloc<str>, str::Utf8Error> {
        let len = libc::strlen(ptr);
        let slice = slice::from_raw_parts(ptr as *const u8, len);
        str::from_utf8(slice).map(|s| Malloc {
            ptr: s as *const str as *mut str,
        })
    }
}

impl<T: ?Sized> Deref for Malloc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T: ?Sized> Drop for Malloc<T> {
    fn drop(&mut self) {
        if (self.ptr as *mut c_void) != DUMMY_PTR {
            unsafe {
                ptr::drop_in_place(self.ptr);
                libc::free(self.ptr as *mut c_void);
            }
        }
    }
}

impl<T: ?Sized> AsRef<T> for Malloc<T> {
    fn as_ref(&self) -> &T {
        &**self
    }
}
