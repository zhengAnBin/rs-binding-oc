use rs_oc_basic::Object;

#[link(name = "WebKit", kind = "framework")]
extern "C" {
    pub static WKWebViewConfiguration: Object;
    pub static WKUserContentController: Object;
    pub static WKPreferences: Object;
}

mod wk_preferences;
mod wk_webview_configuration;
pub use wk_preferences::*;
pub use wk_webview_configuration::*;
