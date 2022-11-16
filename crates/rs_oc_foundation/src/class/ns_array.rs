use rs_oc_basic::{class, msg_send, sel, sel_impl, Object, BOOL};

use crate::NSUInteger;

pub trait NSArray: Sized {
    /// Creates and returns an empty array.
    fn array(_: Self) -> Object {
        unsafe { msg_send![class!(NSArray), array] }
    }

    /// Creates and returns an array containing a given object.
    fn array_with_object(_: Self, object: Object) -> Object {
        unsafe { msg_send![class!(NSArray), arrayWithObject: object] }
    }

    /// Creates and returns an array containing the objects in the argument list.
    fn array_with_objects(_: Self, objects: &[Object]) -> Object {
        unsafe { msg_send![class!(NSArray), arrayWithObjects:objects.as_ptr() count:objects.len()] }
    }

    fn contains_object(self, object: Object) -> BOOL;

    fn count(self) -> NSUInteger;

    fn object_at_index(self, index: NSUInteger) -> Object;

    fn first_object(self) -> Object;

    fn last_object(self) -> Object;
}

impl NSArray for Object {
    /// Returns a Boolean value that indicates whether a given object is present in the array.
    fn contains_object(self, object: Object) -> BOOL {
        unsafe { msg_send![self, containsObject: object] }
    }

    /// The number of objects in the array.
    fn count(self) -> NSUInteger {
        unsafe { msg_send![self, count] }
    }

    /// The first object in the array.
    fn first_object(self) -> Object {
        unsafe { msg_send![self, firstObject] }
    }

    /// The last object in the array.
    fn last_object(self) -> Object {
        unsafe { msg_send![self, lastObject] }
    }

    /// Returns the object located at the specified index.
    fn object_at_index(self, index: NSUInteger) -> Object {
        unsafe { msg_send![self, objectAtIndex: index] }
    }
}
