use rs_oc_basic::Object;

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    pub static NSURL: Object;
}

mod ns_url;

pub use ns_url::*;
