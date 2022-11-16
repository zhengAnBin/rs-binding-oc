use crate::NSUInteger;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

impl NSRange {
    pub fn new(location: NSUInteger, length: NSUInteger) -> NSRange {
        NSRange { location, length }
    }
}
