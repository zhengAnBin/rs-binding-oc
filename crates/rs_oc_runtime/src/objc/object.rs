use super::PrivateMarker;
#[repr(C)]
pub struct Object {
    _priv: PrivateMarker,
}
