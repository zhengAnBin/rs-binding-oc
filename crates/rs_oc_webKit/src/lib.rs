mod web_view;
mod webview_configuration;
pub use web_view::*;
pub use webview_configuration::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
