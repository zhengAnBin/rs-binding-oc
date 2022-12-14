use super::Ivar;
use super::PrivateMarker;
#[repr(C)]
pub struct Object {
    _priv: PrivateMarker,
}

impl Object {
    pub unsafe fn get_ivar<T>(&self, name: &str) -> &T {
        todo!()
    }
}
