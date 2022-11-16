use macos_webview::{WebView, WebViewOptions};

fn main() {
    let webview = WebView::new(WebViewOptions::default());
    webview.load_html("<h1>Hello World</h1>", "");
    webview.run();
}
