use rs_oc_basic::{class, msg_send, sel, sel_impl, Object, BOOL};
use rs_oc_foundation::NSUInteger;

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSApplicationActivationOptions {
    NSApplicationActivateAllWindows = 1 << 0,
    NSApplicationActivateIgnoringOtherApps = 1 << 1,
}

pub trait NSRunningApplication: Sized {
    fn current_application(_: Self) -> Object {
        unsafe { msg_send![class!(NSRunningApplication), currentApplication] }
    }

    fn activate_with_options(self, options: NSApplicationActivationOptions) -> BOOL;
}

impl NSRunningApplication for Object {
    fn activate_with_options(self, options: NSApplicationActivationOptions) -> BOOL {
        unsafe { msg_send![self, activateWithOptions: options as NSUInteger] }
    }
}
