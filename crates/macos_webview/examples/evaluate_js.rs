use macos_webview::{WebView, WebViewOptions};

fn main() {
    let webview = WebView::new(WebViewOptions::default());
    webview.evaluate_js("var h1 = document.createElement('h1'); h1.textContent = '一个H1标签'; document.body.appendChild(h1).toString();");
    webview.run();
}
