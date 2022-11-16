use rs_oc_basic::NIL;
use rs_oc_foundation::{NSRange, NSString};

fn main() {
    let string = NSString::alloc(NIL).init_with_bytes("123455");
    println!("{:?}", string.to_str());

    let prefix = NSString::alloc(NIL).init_with_bytes("123");
    let suffix = NSString::alloc(NIL).init_with_bytes("544");
    println!("{:?}", string.has_suffix(suffix)); // false
    println!("{:?}", string.has_prefix(prefix)); // ture

    let str2 = NSString::alloc(NIL).init_with_bytes("123425");
    println!("{:?}", string.is_equal_to_string(str2)); // false

    let str3 = str2.substring_from_index(2);

    let range = NSRange::new(1, 2);
    let str4 = string.substring_with_range(range);
    println!("substring_with_range for str4: {:?}", str4.to_str());
}
