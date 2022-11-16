use rs_oc_basic::{class, msg_send, sel, sel_impl, Object};
use rs_oc_core_graphics::CGRect;

pub trait WKWebView: Sized {
    fn alloc(_: Self) -> Object {
        unsafe { msg_send![class!(WKWebView), alloc] }
    }

    fn init_with_frame_configuration(self, frame_rect: CGRect, configuration: Object) -> Object;

    fn load_html_string_baseurl(self, string: Object, base_url: Object) -> Object;

    fn load_request(self, request: Object);
}

impl WKWebView for Object {
    fn init_with_frame_configuration(self, frame_rect: CGRect, configuration: Object) -> Object {
        unsafe { msg_send![self, initWithFrame:frame_rect configuration:configuration] }
    }

    fn load_html_string_baseurl(self, string: Object, base_url: Object) -> Object {
        unsafe { msg_send![self, loadHTMLString:string baseURL:base_url] }
    }

    fn load_request(self, request: Object) {
        unsafe { msg_send![self, loadRequest: request] }
    }
}
