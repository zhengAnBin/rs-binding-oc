use std::ffi::c_char;

mod class;
mod encoding;
mod imp;
mod ivar;
mod malloc_buf;
mod method;
mod object;
mod sel;

pub use class::*;
pub use encoding::*;
pub use imp::*;
pub use ivar::*;
pub use malloc_buf::*;
pub use method::*;
pub use object::*;
pub use sel::*;

#[link(name = "objc", kind = "dylib")]
extern "C" {
    pub fn objc_getClass(name: *const c_char) -> *const Class;
    pub fn sel_registerName(name: *const c_char) -> Sel;
    pub fn objc_msgSend();
    pub fn class_getName(cls: *const Class) -> *const c_char;
    pub fn class_getInstanceSize(cls: *const Class) -> usize;
    pub fn ivar_getName(ivar: *const Ivar) -> *const c_char;
    pub fn ivar_getOffset(ivar: *const Ivar) -> isize;
    pub fn ivar_getTypeEncoding(ivar: *const Ivar) -> *const c_char;
    pub fn object_getClass(obj: *const Object) -> *const Class;
    pub fn class_getInstanceVariable(cls: *const Class, name: *const c_char) -> *const Ivar;
    pub fn sel_getName(sel: Sel) -> *const c_char;
    pub fn method_getName(method: *const Method) -> Sel;
}

pub type PrivateMarker = [u8; 0];
