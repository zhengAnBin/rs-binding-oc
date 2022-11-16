use rs_oc_basic::{class, libc::c_char, msg_send, sel, sel_impl, Object, BOOL, UTF8_ENCODING};

use crate::{NSRange, NSUInteger};

pub trait NSString: Sized {
    fn alloc(_: Self) -> Object {
        unsafe { msg_send![class!(NSString), alloc] }
    }

    fn string(_: Self) -> Object {
        unsafe { msg_send![class!(NSString), string] }
    }

    fn string_with_utf8_string(_: Self) -> Object {
        todo!()
        // unsafe { msg_send![class!(NSString), stringWithUTF8String] }
    }

    fn init(self) -> Object;

    fn init_with_bytes(self, string: &str) -> Object;

    fn length(self) -> usize;

    fn utf8_string(self) -> *const c_char;

    fn has_prefix(self, string: Object) -> BOOL;

    fn has_suffix(self, string: Object) -> BOOL;

    fn is_equal_to_string(self, string: Object) -> BOOL;

    fn substring_from_index(self, form: NSUInteger) -> Object;

    fn substring_with_range(self, range: NSRange) -> Object;

    fn to_str(self) -> Result<&'static str, std::str::Utf8Error>;
}

impl NSString for Object {
    fn init(self) -> Object {
        unsafe { msg_send![self, init] }
    }

    fn init_with_bytes(self, string: &str) -> Object {
        unsafe {
            msg_send![self, initWithBytes:string.as_ptr() length:string.len() encoding:UTF8_ENCODING as Object]
        }
    }

    fn length(self) -> usize {
        unsafe { msg_send![self, lengthOfBytesUsingEncoding: UTF8_ENCODING] }
    }

    fn utf8_string(self) -> *const c_char {
        unsafe { msg_send![self, UTF8String] }
    }

    fn has_prefix(self, string: Object) -> BOOL {
        unsafe { msg_send![self, hasPrefix: string] }
    }

    fn has_suffix(self, string: Object) -> BOOL {
        unsafe { msg_send![self, hasSuffix: string] }
    }

    fn is_equal_to_string(self, string: Object) -> BOOL {
        unsafe { msg_send![self, isEqualToString: string] }
    }

    fn substring_from_index(self, form: NSUInteger) -> Object {
        unsafe { msg_send![self, substringFromIndex: form] }
    }

    fn substring_with_range(self, range: NSRange) -> Object {
        unsafe { msg_send![self, substringWithRange: range] }
    }

    fn to_str(self) -> Result<&'static str, std::str::Utf8Error> {
        unsafe {
            std::str::from_utf8(std::slice::from_raw_parts(
                self.utf8_string() as *const u8,
                self.length(),
            ))
        }
        // unsafe {  };
    }
}
