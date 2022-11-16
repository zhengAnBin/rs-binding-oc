use rs_oc_basic::{class, msg_send, sel, sel_impl, Object};

pub trait NSAutoreleasePool: Sized {
    fn new(_: Self) -> Object {
        unsafe { msg_send![class!(NSAutoreleasePool), new] }
    }

    fn autorelease(self) -> Self;
}

impl NSAutoreleasePool for Object {
    fn autorelease(self) -> Object {
        unsafe { msg_send![self, autorelease] }
    }
}
