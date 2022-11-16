use crate::{NSInteger, NSUInteger};
use rs_oc_basic::libc::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulong,
    c_ulonglong, c_ushort,
};
use rs_oc_basic::{class, msg_send, sel, sel_impl, Object, BOOL};

/// An object wrapper for primitive scalar numeric values.
///
/// # Details
/// https://developer.apple.com/documentation/foundation/nsnumber?language=objc
pub trait NSNumber: Sized {
    fn alloc(_: Self) -> Object {
        unsafe { msg_send![class!(NSNumber), alloc] }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a BOOL.
    fn number_with_bool(_: Self, bool: BOOL) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithBool: bool] }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed char.
    fn number_with_char(_: Self, char: c_char) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithChar: char] }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a double.
    fn number_with_double(_: Self, double: c_double) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithDouble: double] }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a float.
    fn number_with_float(_: Self, float: c_float) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithFloat: float] }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed int.
    fn number_with_int(_: Self, int: c_int) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithInt: int] }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an NSInteger.
    fn number_with_integer(_: Self, ns_integer: NSInteger) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithInteger: ns_integer] }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed long.
    fn number_with_long(_: Self, long: c_long) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithLong: long] }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as a signed long long.
    fn number_with_longlong(_: Self, longlong: c_longlong) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithLongLong: longlong] }
    }

    /// Creates and returns an NSNumber object containing value, treating it as a signed short.
    fn number_with_short(_: Self, short: c_short) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithShort: short] }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned char.
    fn number_with_unsigned_char(_: Self, unsigned_char: c_uchar) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithUnsignedChar: unsigned_char] }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned int.
    fn number_with_unsigned_int(_: Self, unsigned_int: c_uint) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithUnsignedInt: unsigned_int] }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an NSUInteger.
    fn number_with_unsigned_integer(_: Self, unsigned_integer: NSUInteger) -> Object {
        unsafe {
            msg_send![
                class!(NSNumber),
                numberWithUnsignedInteger: unsigned_integer
            ]
        }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned long.
    fn number_with_unsigned_long(_: Self, unsigned_long: c_ulong) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithUnsignedLong: unsigned_long] }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned long long.
    fn number_with_unsigned_longlong(_: Self, unsigned_longlong: c_ulonglong) -> Object {
        unsafe {
            msg_send![
                class!(NSNumber),
                numberWithUnsignedLongLong: unsigned_longlong
            ]
        }
    }

    /// Creates and returns an NSNumber object containing a given value, treating it as an unsigned short.
    fn number_with_unsigned_short(_: Self, unsigned_short: c_ushort) -> Object {
        unsafe { msg_send![class!(NSNumber), numberWithUnsignedShort: unsigned_short] }
    }

    fn init_with_bool(self, bool: BOOL) -> Object;

    fn init_with_char(self, char: c_char) -> Object;

    fn init_with_double(self, double: c_double) -> Object;

    fn init_with_float(self, float: c_float) -> Object;

    fn init_with_int(self, int: c_int) -> Object;

    fn init_with_integer(self, integer: NSInteger) -> Object;

    fn init_with_long(self, long: c_long) -> Object;

    fn init_with_longlong(self, longlong: c_longlong) -> Object;

    fn init_with_short(self, short: c_short) -> Object;

    fn init_with_unsigned_char(self, unsigned_char: c_uchar) -> Object;

    fn init_with_unsigned_int(self, unsigned_int: c_uint) -> Object;

    fn init_with_unsigned_integer(self, unsigned_integer: NSUInteger) -> Object;

    fn init_with_unsigned_long(self, unsigned_long: c_ulong) -> Object;

    fn init_with_unsigned_longlong(self, unsigned_longlong: c_ulonglong) -> Object;

    fn init_with_unsigned_short(self, unsigned_short: c_ushort) -> Object;

    fn bool_value(self) -> BOOL;

    fn char_value(self) -> c_char;

    fn double_value(self) -> c_double;

    fn float_value(self) -> c_float;

    fn int_value(self) -> c_int;

    fn integer_value(self) -> NSInteger;

    fn long_value(self) -> c_long;

    fn longlong_value(self) -> c_longlong;

    fn short_value(self) -> c_short;

    fn unsigned_char_value(self) -> c_uchar;

    fn unsigned_int_value(self) -> c_uint;

    fn unsigned_integer_value(self) -> NSUInteger;

    fn unsigned_long_value(self) -> c_ulong;

    fn unsigned_longlong_value(self) -> c_ulonglong;

    fn unsigned_short_value(self) -> c_ushort;
}

impl NSNumber for Object {
    /// Returns an NSNumber object initialized to contain a given value, treated as a BOOL.
    fn init_with_bool(self, bool: BOOL) -> Object {
        unsafe { msg_send![class!(self), initWithBool: bool] }
    }

    fn init_with_char(self, char: c_char) -> Object {
        unsafe { msg_send![class!(self), initWithChar: char] }
    }

    fn init_with_double(self, double: c_double) -> Object {
        unsafe { msg_send![class!(self), initWithChar: double] }
    }

    fn init_with_float(self, float: c_float) -> Object {
        unsafe { msg_send![class!(self), initWithFloat: float] }
    }

    fn init_with_int(self, int: c_int) -> Object {
        unsafe { msg_send![class!(self), initWithInt: int] }
    }

    fn init_with_integer(self, integer: NSInteger) -> Object {
        unsafe { msg_send![class!(self), initWithInteger: integer] }
    }

    fn init_with_long(self, long: c_long) -> Object {
        unsafe { msg_send![class!(self), initWithLong: long] }
    }

    fn init_with_longlong(self, longlong: c_longlong) -> Object {
        unsafe { msg_send![class!(self), initWithLongLong: longlong] }
    }

    fn init_with_short(self, short: c_short) -> Object {
        unsafe { msg_send![class!(self), initWithShort: short] }
    }

    fn init_with_unsigned_char(self, unsigned_char: c_uchar) -> Object {
        unsafe { msg_send![class!(self), initWithUnsignedChar: unsigned_char] }
    }

    fn init_with_unsigned_int(self, unsigned_int: c_uint) -> Object {
        unsafe { msg_send![class!(self), initWithUnsignedInt: unsigned_int] }
    }

    fn init_with_unsigned_integer(self, unsigned_integer: NSUInteger) -> Object {
        unsafe { msg_send![class!(self), initWithUnsignedInteger: unsigned_integer] }
    }

    fn init_with_unsigned_long(self, unsigned_long: c_ulong) -> Object {
        unsafe { msg_send![class!(self), initWithUnsignedLong: unsigned_long] }
    }

    fn init_with_unsigned_longlong(self, unsigned_longlong: c_ulonglong) -> Object {
        unsafe { msg_send![class!(self), initWithUnsignedLongLong: unsigned_longlong] }
    }

    fn init_with_unsigned_short(self, unsigned_short: c_ushort) -> Object {
        unsafe { msg_send![class!(self), initWithUnsignedShort: unsigned_short] }
    }

    fn bool_value(self) -> BOOL {
        unsafe { msg_send![class!(self), boolValue] }
    }

    fn char_value(self) -> c_char {
        unsafe { msg_send![class!(self), charValue] }
    }

    fn double_value(self) -> c_double {
        unsafe { msg_send![class!(self), doubleValue] }
    }

    fn float_value(self) -> c_float {
        unsafe { msg_send![class!(self), floatValue] }
    }

    fn int_value(self) -> c_int {
        unsafe { msg_send![class!(self), intValue] }
    }

    fn integer_value(self) -> NSInteger {
        unsafe { msg_send![class!(self), integerValue] }
    }

    fn long_value(self) -> c_long {
        unsafe { msg_send![class!(self), longValue] }
    }

    fn longlong_value(self) -> c_longlong {
        unsafe { msg_send![class!(self), longLongValue] }
    }

    fn short_value(self) -> c_short {
        unsafe { msg_send![class!(self), shortValue] }
    }

    fn unsigned_char_value(self) -> c_uchar {
        unsafe { msg_send![class!(self), unsignedCharValue] }
    }

    fn unsigned_int_value(self) -> c_uint {
        unsafe { msg_send![class!(self), unsignedIntValue] }
    }

    fn unsigned_integer_value(self) -> NSUInteger {
        unsafe { msg_send![class!(self), unsignedIntegerValue] }
    }

    fn unsigned_long_value(self) -> c_ulong {
        unsafe { msg_send![class!(self), unsignedLongValue] }
    }

    fn unsigned_longlong_value(self) -> c_ulonglong {
        unsafe { msg_send![class!(self), unsignedLongLongValue] }
    }

    fn unsigned_short_value(self) -> c_ushort {
        unsafe { msg_send![class!(self), unsignedShortValue] }
    }
}

#[cfg(test)]
mod ns_number {
    use crate::NSNumber;
    use rs_oc_basic::NIL;

    #[test]
    fn test_bool() {
        assert_eq!(NSNumber::alloc(NIL).init_with_bool(true).bool_value(), true);
        assert_eq!(NSNumber::number_with_bool(NIL, false).bool_value(), false);
    }

    #[test]
    fn test_char() {
        assert_eq!(NSNumber::alloc(NIL).init_with_char(-1).char_value(), -1);
        assert_eq!(NSNumber::number_with_char(NIL, -2).char_value(), -2);
    }

    #[test]
    fn test_double() {
        assert_eq!(
            NSNumber::alloc(NIL)
                .init_with_double(1.9999999999900000999999999999999999999999999999999999999)
                .double_value(),
            1.9999999999900000999999999999999999999999999999999999999
        );
        assert_eq!(NSNumber::number_with_double(NIL, 2.).double_value(), 2.);
    }

    #[test]
    fn test_float() {
        assert_eq!(
            NSNumber::alloc(NIL).init_with_float(1.123).float_value(),
            1.123
        );
        assert_eq!(NSNumber::number_with_float(NIL, 2.).float_value(), 2.);
    }

    #[test]
    fn test_int() {
        let number = 123456;
        let ns_number = NSNumber::number_with_int(NIL, number);
        let objc_int = ns_number.int_value();
        assert_eq!(number, objc_int);
    }

    #[test]
    fn test_integer() {
        assert_eq!(NSNumber::alloc(NIL).init_with_integer(1).integer_value(), 1);
        assert_eq!(NSNumber::number_with_integer(NIL, 2).integer_value(), 2);
    }

    #[test]
    fn test_long() {
        assert_eq!(NSNumber::alloc(NIL).init_with_long(1).long_value(), 1);
        assert_eq!(NSNumber::number_with_long(NIL, 2).long_value(), 2);
    }

    #[test]
    fn test_longlong() {
        assert_eq!(
            NSNumber::alloc(NIL).init_with_longlong(1).longlong_value(),
            1
        );
        assert_eq!(NSNumber::number_with_longlong(NIL, 2).longlong_value(), 2);
    }

    #[test]
    fn test_short() {
        assert_eq!(NSNumber::alloc(NIL).init_with_short(1).short_value(), 1);
        assert_eq!(NSNumber::number_with_short(NIL, 2).short_value(), 2);
    }

    #[test]
    fn test_unsigned_char() {
        assert_eq!(
            NSNumber::alloc(NIL)
                .init_with_unsigned_char(1)
                .unsigned_char_value(),
            1
        );
        assert_eq!(
            NSNumber::number_with_unsigned_char(NIL, 2).unsigned_char_value(),
            2
        );
    }

    #[test]
    fn test_unsigned_int() {
        assert_eq!(
            NSNumber::alloc(NIL)
                .init_with_unsigned_int(1)
                .unsigned_int_value(),
            1
        );
        assert_eq!(
            NSNumber::number_with_unsigned_int(NIL, 2).unsigned_int_value(),
            2
        );
    }

    #[test]
    fn test_unsigned_integer() {
        assert_eq!(
            NSNumber::alloc(NIL)
                .init_with_unsigned_integer(1)
                .unsigned_integer_value(),
            1
        );
        assert_eq!(
            NSNumber::number_with_unsigned_integer(NIL, 2).unsigned_integer_value(),
            2
        );
    }

    #[test]
    fn test_unsigned_long() {
        assert_eq!(
            NSNumber::alloc(NIL)
                .init_with_unsigned_long(1)
                .unsigned_long_value(),
            1
        );
        assert_eq!(
            NSNumber::number_with_unsigned_long(NIL, 2).unsigned_long_value(),
            2
        );
    }

    #[test]
    fn test_unsigned_longlong() {
        assert_eq!(
            NSNumber::alloc(NIL)
                .init_with_unsigned_longlong(1)
                .unsigned_longlong_value(),
            1
        );
        assert_eq!(
            NSNumber::number_with_unsigned_longlong(NIL, 2).unsigned_longlong_value(),
            2
        );
    }

    #[test]
    fn test_unsigned_short() {
        assert_eq!(
            NSNumber::alloc(NIL)
                .init_with_unsigned_short(1)
                .unsigned_short_value(),
            1
        );
        assert_eq!(
            NSNumber::number_with_unsigned_short(NIL, 2).unsigned_short_value(),
            2
        );
    }
}
