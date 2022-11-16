use macos_webview::{WebView, WebViewOptions};

fn main() {
    let webview = WebView::new(WebViewOptions::default());
    webview.load_url("http:://localhost:3000");
    webview.run();
}
