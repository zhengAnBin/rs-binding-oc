use super::method_getName;
use super::{PrivateMarker, Sel};

#[repr(C)]
pub struct Method {
    _priv: PrivateMarker,
}

impl Method {
    pub fn name(&self) -> Sel {
        unsafe { method_getName(self) }
    }
}
