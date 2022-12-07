use crate::objc::{Class, Sel};
use std::os::raw::{c_char, c_int, c_uint, c_void};

#[link(name = "objc", kind = "dylib")]
extern "C" {
    pub fn objc_getClass(name: *const c_char) -> *const Class;
    pub fn sel_registerName(name: *const c_char) -> Sel;
    pub fn objc_msgSend();
}
