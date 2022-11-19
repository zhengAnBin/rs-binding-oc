use macos_webview::{WebView, WebViewOptions};
use rs_oc_basic::Object;

fn main() {
    let webview = WebView::new(WebViewOptions::default());
    let mut callback = |_: Object, _: Object| {
        webview.evaluate_js(
            "
            var div = document.createElement('div');
            div.textContent = Math.random() + '';
            document.body.appendChild(div).toString()
        ",
        );
    };
    webview.add_handler("call_fn", &mut callback);
    webview.load_html(
        "
        <button onClick='jsFunction()'>add node</button>
        <script>
            function jsFunction() {
                window.webkit.messageHandlers.call_fn.postMessage(Math.random() + '');
            }
        </script>
    ",
        "",
    );
    webview.run();
}
