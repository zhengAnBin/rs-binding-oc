use std::ops::Deref;

use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    pub static NSNumber: *mut Object;
}

type id = *mut Object;

struct RSNSNumber(id);

impl Deref for RSNSNumber {
    type Target = id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

trait NSNumber: Sized {
    fn number_with_int(n: i32) -> RSNSNumber {
        unsafe { RSNSNumber(msg_send![NSNumber, numberWithInt: n]) }
    }

    fn int_value(&self) -> i32;
}

impl NSNumber for RSNSNumber {
    fn int_value(&self) -> i32 {
        unsafe { msg_send![*self.deref(), intValue] }
    }
}

fn main() {
    let ns_number = RSNSNumber::number_with_int(1);
    let res = ns_number.int_value();
    println!("res: {:?}", res)
}
