use rs_oc_basic::Object;

#[link(name = "AppKit", kind = "framework")]
extern "C" {
    pub static NSApplication: Object;
    pub static NSWindow: Object;
}
