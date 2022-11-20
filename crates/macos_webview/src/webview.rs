use crate::{make_handler, register_ns_app_lication_delegate, RS_TRAIT_PTR};
use rs_oc_appKit::{
    NSApplication, NSApplicationActivationOptions, NSApplicationActivationPolicy,
    NSApplicationDelegate, NSAutoresizingMaskOptions, NSBackingStoreType, NSRunningApplication,
    NSView, NSWindow, NSWindowStyleMask,
};
use rs_oc_basic::{block, msg_send, sel, sel_impl, Object, NIL, NO};
use rs_oc_core_graphics::{CGFloat, CGPoint, CGRect, CGSize};
use rs_oc_foundation::{NSAutoreleasePool, NSString, NSURLRequest, NSURL};
use rs_oc_webKit::{WKUserContentController, WKWebView, WKWebViewConfiguration};

pub struct WebViewOptions {
    width: CGFloat,
    height: CGFloat,
    x: CGFloat,
    y: CGFloat,
    title: String,
}

impl Default for WebViewOptions {
    fn default() -> Self {
        Self {
            width: 500.,
            height: 300.,
            x: 0.,
            y: 0.,
            title: String::from("webview"),
        }
    }
}

pub struct WebView {
    ns_app: Object,
    main_window: Object,
    webview: Object,
    user_content_controller: Object,
}

impl WebView {
    pub fn new(webview_options: WebViewOptions) -> Self {
        let _pool = NSAutoreleasePool::new(NIL);

        let ns_app = NSApplication::shared_application(NIL);
        ns_app.set_activation_policy(
            NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        );

        let cg_rect = CGRect::new(
            &CGPoint::new(webview_options.x, webview_options.y),
            &CGSize::new(webview_options.width, webview_options.height),
        );
        let main_window = NSWindow::alloc(NIL)
            .init_with_content_rect_stylemask_backing_defer(
                cg_rect,
                NSWindowStyleMask::NSTitledWindowMask,
                NSBackingStoreType::NSBackingStoreBuffered,
                NO,
            )
            .autorelease();
        main_window.center();
        let title = NSString::alloc(NIL).init_with_bytes(&webview_options.title);
        main_window.set_title(title);
        main_window.make_key_and_order_front(NIL);

        // 创建webview
        let webview = WKWebView::alloc(NIL);
        let frame = NSWindow::frame(main_window);
        let configuration = WKWebViewConfiguration::init(WKWebViewConfiguration::alloc(NIL));
        let user_content_controller =
            WKUserContentController::init(WKUserContentController::alloc(NIL));
        configuration.set_user_content_controller(user_content_controller);
        webview.init_with_frame_configuration(frame, configuration);

        NSView::set_autoresizing_mask(webview, NSAutoresizingMaskOptions::NSViewWidthSizable);
        main_window.set_content_view(webview);
        WebView {
            ns_app,
            main_window,
            webview,
            user_content_controller,
        }
    }

    pub fn run(&self) {
        let current_app = NSRunningApplication::current_application(NIL);
        current_app.activate_with_options(
            NSApplicationActivationOptions::NSApplicationActivateIgnoringOtherApps,
        );
        self.ns_app.run();
    }

    pub fn get_webview_handler(self) -> Object {
        self.webview
    }

    pub fn get_main_window_handler(self) -> Object {
        self.main_window
    }

    pub fn get_app_handler(self) -> Object {
        self.ns_app
    }

    pub fn load_html(&self, html: &str, base_url: &str) {
        let html = NSString::alloc(NIL).init_with_bytes(html);
        let base_url = NSString::alloc(NIL).init_with_bytes(base_url);
        let base_url = NSURL::alloc(NIL).init_with_string(base_url);
        self.webview.load_html_string_baseurl(html, base_url);
    }

    pub fn load_url(&self, url: &str) {
        let url = NSString::alloc(NIL).init_with_bytes(url);
        let url = NSURL::alloc(NIL).init_with_string(url);
        let req = NSURLRequest::alloc(NIL).init_with_url(url);
        self.webview.load_request(req);
    }

    pub fn evaluate_js(&self, code: &str) {
        let script = NSString::alloc(NIL).init_with_bytes(code);
        let b = |_: Object, error: Object| {
            if error != NIL {
                unsafe {
                    let ns_str: Object = msg_send![error, localizedDescription];
                    let str = nsstring_to_string(ns_str);
                    println!("Error {}", str.as_ref().unwrap().as_str());
                    return;
                }
            }
        };
        let b = block::ConcreteBlock::new(b);
        let b = b.copy();
        self.webview
            .evaluate_javascript_completion_handler(script, &b);
    }

    pub fn add_handler<Handler>(&self, name: &str, handler: *mut Handler)
    where
        Handler: FnMut(Object, Object),
    {
        let handler = unsafe { make_handler(format!("DWKHandler_{}", name).as_str(), handler) };
        let name = NSString::alloc(NIL).init_with_bytes(name);
        self.user_content_controller
            .add_script_message_handler(handler, name);
    }

    pub fn set_app_delegate<T>(&self, delegate_trait: T)
    where
        T: NSApplicationDelegate + 'static,
    {
        unsafe {
            let app_delegate = Box::new(delegate_trait);
            let delegate_class = register_ns_app_lication_delegate::<T>();
            let delegate: Object = msg_send![delegate_class, new];
            let delegate_ptr: *const T = &*app_delegate;
            (&mut *delegate).set_ivar(RS_TRAIT_PTR, delegate_ptr as usize);
            self.ns_app.set_delegate(delegate);
        }
    }
}

pub unsafe fn nsstring_to_string(nsstring: Object) -> *mut String {
    let len = nsstring.length();
    let str = Box::new(String::from_utf8_unchecked(Vec::from_raw_parts(
        nsstring.utf8_string() as *mut u8,
        len,
        len,
    )));
    Box::into_raw(str)
}
