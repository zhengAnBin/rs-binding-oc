use super::{class_getInstanceSize, class_getInstanceVariable, class_getName, objc_getClass};
use super::{Ivar, PrivateMarker};
use std::ffi::{CStr, CString};
use std::str;

#[repr(C)]
pub struct Class {
    _priv: PrivateMarker,
}

/// OC 类
impl Class {
    /// 使用类名获取类
    pub fn get(name: &str) -> Option<&'static Class> {
        let name = CString::new(name).unwrap();
        unsafe {
            let cls = objc_getClass(name.as_ptr());
            if cls.is_null() {
                None
            } else {
                Some(&*cls)
            }
        }
    }

    /// 返回类名
    pub fn name(&self) -> &str {
        let name = unsafe { CStr::from_ptr(class_getName(self)) };
        str::from_utf8(name.to_bytes()).unwrap()
    }

    /// 获取类的实例大小
    pub fn instance_size(&self) -> usize {
        unsafe { class_getInstanceSize(self) }
    }

    /// 返回类的实例上的成员变量
    pub fn instance_variable(&self, name: &str) -> Option<&Ivar> {
        let name = CString::new(name).unwrap();
        unsafe {
            let ivar = class_getInstanceVariable(self, name.as_ptr());
            if ivar.is_null() {
                None
            } else {
                Some(&*ivar)
            }
        }
    }
}
