use macos_webview::{WebView, WebViewOptions};

fn main() {
    // todo: 开发上线时，需要配置Info.plist才能发起http请求
    // embed_plist::embed_info_plist!("./Info.plist");
    // let embedded_plist = embed_plist::get_info_plist();
    // println!("{:?}", std::str::from_utf8(embedded_plist).unwrap());
    let webview = WebView::new(WebViewOptions::default());
    webview.load_url("https://www.zhihu.com");
    webview.run();
}
