mod class;
mod imp;
mod object;
mod sel;
use std::os::raw::c_char;

pub use class::*;
pub use imp::*;
pub use object::*;
pub use sel::*;

#[link(name = "objc", kind = "dylib")]
extern "C" {
    pub fn objc_getClass(name: *const c_char) -> *const Class;
    pub fn sel_registerName(name: *const c_char) -> Sel;
    pub fn objc_msgSend();
    pub fn class_getName(cls: *const Class) -> *const c_char;
    pub fn class_getInstanceSize(cls: *const Class) -> usize;
}

pub type PrivateMarker = [u8; 0];
