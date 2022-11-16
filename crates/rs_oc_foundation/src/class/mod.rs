use rs_oc_basic::libc;

mod ns_array;
mod ns_autorelease_pool;
mod ns_number;
mod ns_range;
mod ns_string;

pub use ns_array::NSArray;
pub use ns_autorelease_pool::NSAutoreleasePool;
pub use ns_number::NSNumber;
pub use ns_range::NSRange;
pub use ns_string::NSString;

#[cfg(target_pointer_width = "32")]
pub type NSInteger = libc::c_int;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;
