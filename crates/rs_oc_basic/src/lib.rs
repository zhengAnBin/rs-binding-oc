use objc::runtime;

pub use objc::runtime::{BOOL, NO, YES};
pub type Class = *mut runtime::Class;
pub type Object = *mut runtime::Object;
pub type Sel = runtime::Sel;
pub const NIL: Object = 0 as Object;
pub use libc;
pub use objc::{class, msg_send, sel, sel_impl};
pub const UTF8_ENCODING: usize = 4;
pub use block;
