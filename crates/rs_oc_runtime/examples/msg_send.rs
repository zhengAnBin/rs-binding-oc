use objc::runtime::Sel;
use rs_oc_runtime::objc::{Imp, Object};
use std::any::Any;
use std::error::Error;
use std::{fmt, mem};

pub trait MessageArguments: Sized {
    unsafe fn invoke<R>(imp: Imp, obj: *mut Object, sel: Sel, args: Self) -> R
    where
        R: Any;
}

// 完成一个宏，可以传入任意数量的参数，然后调用 invoke 方法
macro_rules! message_args_impl {
    ($($a:ident : $t:ident),*) => (
        impl<$($t),*> MessageArguments for ($($t,)*) {
            unsafe fn invoke<R>(imp: Imp, obj: *mut Object, sel: Sel, ($($a,)*): Self) -> R
             where R: Any {
                let imp: unsafe extern fn(*mut Object, Sel $(, $t)*) -> R = mem::transmute(imp);
                imp(obj, sel $(, $a)*)
            }
        }
    );
}

// 使得 (...args) 可以调用 invoke 方法
message_args_impl!();
message_args_impl!(a: A);
message_args_impl!(a: A, b: B);
message_args_impl!(a: A, b: B, c: C);
message_args_impl!(a: A, b: B, c: C, d: D);
message_args_impl!(a: A, b: B, c: C, d: D, e: E);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);

#[derive(Debug)]
pub struct MessageError(String);

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Error for MessageError {
    fn description(&self) -> &str {
        &self.0
    }
}

fn main() {}
