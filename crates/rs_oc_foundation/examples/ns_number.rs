use rs_oc_basic::NIL;
use rs_oc_foundation::NSNumber;

fn main() {
    let num1 = NSNumber::number_with_int(NIL, 1234);
    println!("{:?}", num1);
}
