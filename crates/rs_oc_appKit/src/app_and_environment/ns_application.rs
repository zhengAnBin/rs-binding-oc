use crate::NSApplicationDelegate;
use rs_oc_basic::{class, msg_send, sel, sel_impl, Object, BOOL};

#[repr(i64)]
pub enum NSApplicationActivationPolicy {
    NSApplicationActivationPolicyRegular = 0,
    NSApplicationActivationPolicyAccessory = 1,
    NSApplicationActivationPolicyProhibited = 2,
    NSApplicationActivationPolicyERROR = -1,
}

pub trait NSApplication: Sized {
    fn shared_application(_: Self) -> Object {
        unsafe { msg_send![class!(NSApplication), sharedApplication] }
    }

    fn set_activation_policy(self, policy: NSApplicationActivationPolicy) -> BOOL;

    fn run(self);

    fn set_delegate(self, delegate: Object);
}

impl NSApplication for Object {
    fn set_activation_policy(self, policy: NSApplicationActivationPolicy) -> BOOL {
        unsafe { msg_send![self, setActivationPolicy: policy] }
    }

    fn run(self) {
        unsafe { msg_send![self, run] }
    }

    fn set_delegate(self, delegate: Object) {
        unsafe { msg_send![self, setDelegate: delegate] }
    }
}
