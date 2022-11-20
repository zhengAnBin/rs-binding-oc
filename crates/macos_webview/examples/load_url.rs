use macos_webview::{WebView, WebViewOptions};

fn main() {
    let webview = WebView::new(WebViewOptions::default());
    // todo: 需要配置Info.plist才能发起请求
    webview.load_url("https:://baidu.com");
    webview.run();
}
