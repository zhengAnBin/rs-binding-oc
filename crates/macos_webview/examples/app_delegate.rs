use macos_webview::{WebView, WebViewOptions};
use rs_oc_appKit::NSApplicationDelegate;

struct AppDelegate;

impl NSApplicationDelegate for AppDelegate {
    fn application_will_finish_launching(&self) {
        println!("应用程序将完成启动")
    }
}

fn main() {
    let webview = WebView::new(WebViewOptions::default());
    webview.set_app_delegate(AppDelegate {});
    webview.load_html("<h1>Hello World</h1>", "");
    webview.run();
}
