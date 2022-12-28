#[allow(unused)]
#[napi_derive::napi]
fn run(args: Vec<String>, bin_name: Option<String>) {
    create_macos_webview::run(args, bin_name);
}
