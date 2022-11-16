use rs_oc_basic::Object;

#[link(name = "WebKit", kind = "framework")]
extern "C" {
    pub static WKWebView: Object;
}

mod wk_web_view;

pub use wk_web_view::*;
