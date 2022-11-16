use rs_oc_basic::{class, msg_send, sel, sel_impl, Object};

/// https://developer.apple.com/documentation/webkit/wkusercontentcontroller?language=objc
pub trait WKUserContentController: Sized {
    fn alloc(_: Self) -> Object {
        unsafe { msg_send![class!(WKUserContentController), alloc] }
    }

    fn init(self) -> Object;

    fn add_script_message_handler(self, message_handler: Object, name: Object) -> Object;
}

impl WKUserContentController for Object {
    fn init(self) -> Object {
        unsafe { msg_send![self, init] }
    }

    fn add_script_message_handler(self, message_handler: Object, name: Object) -> Object {
        unsafe { msg_send![self, addScriptMessageHandler: message_handler name: name] }
    }
}

/// https://developer.apple.com/documentation/webkit/wkwebviewconfiguration?language=objc
pub trait WKWebViewConfiguration: Sized {
    fn alloc(_: Self) -> Object {
        unsafe { msg_send![class!(WKWebViewConfiguration), alloc] }
    }

    fn init(self) -> Object;

    fn set_user_content_controller(self, controller: Object);
}

impl WKWebViewConfiguration for Object {
    fn init(self) -> Object {
        unsafe { msg_send![self, init] }
    }

    fn set_user_content_controller(self, controller: Object) {
        unsafe { msg_send![self, setUserContentController: controller] }
    }
}
