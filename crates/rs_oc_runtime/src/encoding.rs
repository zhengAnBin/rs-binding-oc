//! 实现 objective-c 的类型编码
//! [苹果文档介绍](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html)

use std::ffi::c_void;
use std::fmt;

#[derive(Debug)]
pub enum Encoding<'a> {
    Char,
    Short,
    Int,
    Long,
    LongLong,
    UChar,
    UShort,
    UInt,
    ULong,
    ULongLong,
    Float,
    Double,
    Bool,
    Void,
    String,
    Object,
    Block,
    Class,
    Sel,
    Unknown,
    BitField(u32),
    Pointer(&'a Encoding<'a>),
    Array(u32, &'a Encoding<'a>),
    Struct(&'a str, &'a [Encoding<'a>]),
    Union(&'a str, &'a [Encoding<'a>]),
}

impl fmt::Display for Encoding<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use Encoding::*;
        formatter.write_str(match *self {
            Char => "c",
            Short => "s",
            Int => "i",
            Long => "l",
            LongLong => "q",
            UChar => "C",
            UShort => "S",
            UInt => "I",
            ULong => "L",
            ULongLong => "Q",
            Float => "f",
            Double => "d",
            Bool => "B",
            Void => "v",
            String => "*",
            Object => "@",
            Block => "@?",
            Class => "#",
            Sel => ":",
            Unknown => "?",
            BitField(b) => {
                return write!(formatter, "b{}", b);
            }
            Pointer(t) => {
                return write!(formatter, "^{}", t);
            }
            Array(len, item) => {
                return write!(formatter, "[{}{}]", len, item);
            }
            Struct(name, fields) => {
                write!(formatter, "{{{}=", name)?;
                for field in fields {
                    fmt::Display::fmt(field, formatter)?;
                }
                return formatter.write_str("}");
            }
            Union(name, members) => {
                write!(formatter, "({}=", name)?;
                for member in members {
                    fmt::Display::fmt(member, formatter)?;
                }
                return formatter.write_str(")");
            }
        })
    }
}

// fn ep(s: &str, encode: &Encoding) -> bool {
//     let s = s.trim_start_matches(QUALIFIERS);

// }

impl PartialEq<str> for Encoding<'_> {
    fn eq(&self, other: &str) -> bool {
        // TODO: 如果直接使用 == 来比较，会导致栈溢出。
        // self == other
        todo!()
    }
}

impl PartialEq<Encoding<'_>> for str {
    fn eq(&self, other: &Encoding) -> bool {
        // TODO: 如果直接使用 == 来比较，会导致栈溢出。
        // self == other
        todo!()
    }
}

/// 为rust类型实现编码，使得rust类与oc类型可以互相转换
pub trait Encode {
    const ENCODING: Encoding<'static>;
}

impl Encode for i16 {
    const ENCODING: Encoding<'static> = Encoding::Short;
}

impl Encode for i32 {
    const ENCODING: Encoding<'static> = Encoding::Int;
}

impl Encode for i64 {
    const ENCODING: Encoding<'static> = Encoding::LongLong;
}

impl Encode for u8 {
    const ENCODING: Encoding<'static> = Encoding::UChar;
}

impl Encode for u16 {
    const ENCODING: Encoding<'static> = Encoding::UShort;
}

impl Encode for u32 {
    const ENCODING: Encoding<'static> = Encoding::UInt;
}

impl Encode for u64 {
    const ENCODING: Encoding<'static> = Encoding::ULongLong;
}

impl Encode for f32 {
    const ENCODING: Encoding<'static> = Encoding::Float;
}

impl Encode for f64 {
    const ENCODING: Encoding<'static> = Encoding::Double;
}

impl Encode for bool {
    const ENCODING: Encoding<'static> = Encoding::Bool;
}

impl Encode for () {
    const ENCODING: Encoding<'static> = Encoding::Void;
}

impl Encode for *mut i8 {
    const ENCODING: Encoding<'static> = Encoding::String;
}

impl Encode for *const i8 {
    const ENCODING: Encoding<'static> = Encoding::String;
}

impl Encode for *mut u8 {
    const ENCODING: Encoding<'static> = Encoding::String;
}

impl Encode for *const u8 {
    const ENCODING: Encoding<'static> = Encoding::String;
}

impl Encode for isize {
    #[cfg(target_pointer_width = "32")]
    const ENCODING: Encoding<'static> = i32::ENCODING;

    #[cfg(target_pointer_width = "64")]
    const ENCODING: Encoding<'static> = i64::ENCODING;
}

impl Encode for usize {
    #[cfg(target_pointer_width = "32")]
    const ENCODING: Encoding<'static> = u32::ENCODING;

    #[cfg(target_pointer_width = "64")]
    const ENCODING: Encoding<'static> = u64::ENCODING;
}

impl Encode for *mut c_void {
    const ENCODING: Encoding<'static> = Encoding::Pointer(&Encoding::Void);
}

impl Encode for *const c_void {
    const ENCODING: Encoding<'static> = Encoding::Pointer(&Encoding::Void);
}

impl<T: Encode, const N: usize> Encode for [T; N] {
    const ENCODING: Encoding<'static> = Encoding::Array(N as u32, &<T as Encode>::ENCODING);
}

impl<T> Encode for *const T
where
    for<'b> &'b T: Encode,
{
    const ENCODING: Encoding<'static> = <&T>::ENCODING;
}

impl<T> Encode for *mut T
where
    for<'b> &'b mut T: Encode,
{
    const ENCODING: Encoding<'static> = <&mut T>::ENCODING;
}

impl<'a, T> Encode for Option<&'a T>
where
    for<'b> &'b T: Encode,
{
    const ENCODING: Encoding<'static> = <&T>::ENCODING;
}

impl<'a, T> Encode for Option<&'a mut T>
where
    for<'b> &'b mut T: Encode,
{
    const ENCODING: Encoding<'static> = <&mut T>::ENCODING;
}
