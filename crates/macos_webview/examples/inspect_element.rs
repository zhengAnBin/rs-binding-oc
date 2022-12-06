use macos_webview::{WebView, WebViewOptions};

fn main() {
    let mut webview_options = WebViewOptions::default();
    webview_options.debug = true;
    let webview = WebView::new(webview_options);
    webview.load_html("<h1>inspect element</h1>", "");
    webview.run();
}
