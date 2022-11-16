use rs_oc_basic::Object;

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    pub static NSString: Object;
    pub static NSNumber: Object;
    pub static NSArray: Object;
    pub static NSRange: Object;
    pub static NSAutoreleasePool: Object;
    pub static NSURLRequest: Object;
}
