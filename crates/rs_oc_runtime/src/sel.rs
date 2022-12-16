use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::str;

use super::{sel_getName, sel_registerName};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Sel {
    ptr: *const c_void,
}

impl Sel {
    /// 注册一个 Sel
    pub fn register(name: &str) -> Sel {
        let name = CString::new(name).unwrap();
        unsafe { sel_registerName(name.as_ptr()) }
    }

    /// 返回 Sel 名
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(sel_getName(*self)) };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// 返回 Sel 的原始指针
    pub fn as_ptr(&self) -> *const c_void {
        self.ptr
    }
}
