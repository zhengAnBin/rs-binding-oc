use rs_oc_basic::{msg_send, sel, sel_impl, Object};
use rs_oc_foundation::NSUInteger;

pub enum NSAutoresizingMaskOptions {
    NSViewWidthSizable = 2,
}

pub trait NSView: Sized {
    fn set_autoresizing_mask(self, autoresizing_mask: NSAutoresizingMaskOptions);
}

impl NSView for Object {
    fn set_autoresizing_mask(self, autoresizing_mask: NSAutoresizingMaskOptions) {
        unsafe { msg_send![self, setAutoresizingMask: autoresizing_mask as NSUInteger] }
    }
}
