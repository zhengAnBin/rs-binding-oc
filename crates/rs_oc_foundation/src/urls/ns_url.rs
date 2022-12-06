use rs_oc_basic::{class, msg_send, sel, sel_impl, Object};

pub trait NSURL: Sized {
    fn alloc(_: Self) -> Object {
        unsafe { msg_send![class!(NSURL), alloc] }
    }

    fn init_with_string(self, string: Object) -> Object;
}

impl NSURL for Object {
    fn init_with_string(self, string: Object) -> Object {
        unsafe { msg_send![self, initWithString: string] }
    }
}

pub trait NSURLRequest: Sized {
    fn alloc(_: Self) -> Object {
        unsafe { msg_send![class!(NSURLRequest), alloc] }
    }

    fn init_with_url(self, url: Object) -> Object;

    fn request_with_url(_: Self, url: Object) -> Object {
        unsafe { msg_send![class!(NSURLRequest), requestWithURL: url] }
    }
}

impl NSURLRequest for Object {
    fn init_with_url(self, url: Object) -> Object {
        unsafe { msg_send![self, initWithURL: url] }
    }
}
