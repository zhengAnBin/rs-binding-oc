use std::os::raw::c_void;
type PrivateMarker = [u8; 0];

#[repr(C)]
pub struct Class {
    _priv: PrivateMarker,
}

#[derive(Debug)]
#[repr(C)]
pub struct Sel {
    ptr: *const c_void,
}

pub type Imp = unsafe extern "C" fn();
