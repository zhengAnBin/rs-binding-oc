use rs_oc_basic::libc;

/// https://developer.apple.com/documentation/corefoundation/cgfloat?language=objc
#[cfg(target_pointer_width = "64")]
pub type CGFloat = libc::c_double;
#[cfg(not(target_pointer_width = "64"))]
pub type CGFloat = libc::c_float;

/// https://developer.apple.com/documentation/corefoundation/cgsize?language=objc
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl CGSize {
    pub fn new(width: CGFloat, height: CGFloat) -> CGSize {
        CGSize { width, height }
    }
}

/// https://developer.apple.com/documentation/corefoundation/cgpoint?language=objc
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

impl CGPoint {
    pub fn new(x: CGFloat, y: CGFloat) -> CGPoint {
        CGPoint { x, y }
    }
}

/// https://developer.apple.com/documentation/corefoundation/cgrect?language=objc
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

impl CGRect {
    pub fn new(origin: &CGPoint, size: &CGSize) -> CGRect {
        CGRect {
            origin: *origin,
            size: *size,
        }
    }
}

/// https://developer.apple.com/documentation/corefoundation/cgvector?language=objc
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CGVector {
    dx: CGFloat,
    dy: CGFloat,
}

impl CGVector {
    pub fn new(dx: CGFloat, dy: CGFloat) -> CGVector {
        CGVector { dx, dy }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
