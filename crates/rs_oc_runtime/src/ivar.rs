use std::ffi::CStr;
use std::str;

use super::PrivateMarker;
use super::{ivar_getName, ivar_getOffset};

#[repr(C)]
pub struct Ivar {
    _priv: PrivateMarker,
}

impl Ivar {
    /// 返回成员变量名
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(ivar_getName(self)) };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// 返回成员变量的偏移量
    pub fn offset(&self) -> isize {
        unsafe { ivar_getOffset(self) }
    }

    /// 返回成员变量的类型编码
    pub fn type_encoding(&self) -> &str {
        let name = unsafe { CStr::from_ptr(super::ivar_getTypeEncoding(self)) };
        str::from_utf8(name.to_bytes()).unwrap()
    }
}
