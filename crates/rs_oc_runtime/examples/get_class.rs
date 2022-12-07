use rs_oc_runtime::objc::Class;
use rs_oc_runtime::runtime::{objc_getClass, sel_registerName};

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    pub static NSArray: *mut Class;
}

fn main() {
    unsafe {
        let class = objc_getClass(concat!(stringify!(NSArray), "\0").as_ptr() as *const i8);
        let sel = sel_registerName(concat!(stringify!(alloc), "\0").as_ptr() as *const i8);
        // todo: use objc_msgSend
        println!("class: {:?}", class);
        println!("sel: {:?}", sel);
    }
}
