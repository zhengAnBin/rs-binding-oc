use rs_oc_basic::Object;

#[link(name = "AppKit", kind = "framework")]
extern "C" {
    pub static NSView: Object;
}

mod ns_view;

pub use ns_view::*;
