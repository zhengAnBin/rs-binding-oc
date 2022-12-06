use rs_oc_basic::{class, msg_send, sel, sel_impl, Object};

pub trait WKPreferences: Sized {
    fn alloc(_: Self) -> Object {
        unsafe { msg_send![class!(WKPreferences), alloc] }
    }

    fn init(self) -> Object;

    fn set_value_for_key(self, value: Object, for_key: Object);
}

impl WKPreferences for Object {
    fn init(self) -> Object {
        unsafe { msg_send![self, init] }
    }

    fn set_value_for_key(self, value: Object, for_key: Object) {
        unsafe { msg_send![self, setValue: value forKey: for_key] }
    }
}
