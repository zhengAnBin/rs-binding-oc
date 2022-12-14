use super::PrivateMarker;
use super::{class_getInstanceSize, class_getName, objc_getClass};
use std::ffi::{CStr, CString};
use std::str;

#[repr(C)]
pub struct Class {
    _priv: PrivateMarker,
}

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
}
