use std::os::raw::c_void;

#[derive(Debug)]
#[repr(C)]
pub struct Sel {
    ptr: *const c_void,
}
