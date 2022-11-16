use rs_oc_basic::{class, msg_send, sel, sel_impl, Object, BOOL};
use rs_oc_core_graphics::{CGPoint, CGRect};
use rs_oc_foundation::NSUInteger;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSWindowStyleMask {
    NSTitledWindowMask = 1 << 0,
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSBackingStoreType {
    NSBackingStoreRetained = 0,
    NSBackingStoreNonretained = 1,
    NSBackingStoreBuffered = 2,
}

pub trait NSWindow: Sized {
    fn alloc(_: Self) -> Object {
        unsafe { msg_send![class!(NSWindow), alloc] }
    }

    fn init_with_content_rect_stylemask_backing_defer(
        self,
        rect: CGRect,
        style: NSWindowStyleMask,
        backing: NSBackingStoreType,
        defer: BOOL,
    ) -> Object;

    fn make_key_and_order_front(self, sender: Object) -> Object;

    fn set_title(self, title: Object) -> Object;

    fn cascade_top_left_from_point(self, top_left: CGPoint) -> Object;

    fn center(self) -> Object;

    fn frame(self) -> CGRect;

    fn set_content_view(self, view: Object);
}

impl NSWindow for Object {
    fn init_with_content_rect_stylemask_backing_defer(
        self,
        rect: CGRect,
        style: NSWindowStyleMask,
        backing: NSBackingStoreType,
        defer: BOOL,
    ) -> Object {
        unsafe {
            msg_send![self, initWithContentRect:rect
            styleMask:style as NSUInteger
              backing:backing as NSUInteger
                defer:defer]
        }
    }

    fn make_key_and_order_front(self, sender: Object) -> Object {
        unsafe { msg_send![self, makeKeyAndOrderFront: sender] }
    }

    fn set_title(self, title: Object) -> Object {
        unsafe { msg_send![self, setTitle: title] }
    }

    fn cascade_top_left_from_point(self, top_left: CGPoint) -> Object {
        unsafe { msg_send![self, cascadeTopLeftFromPoint: top_left] }
    }

    fn center(self) -> Object {
        unsafe { msg_send![self, center] }
    }

    fn frame(self) -> CGRect {
        unsafe { msg_send![self, frame] }
    }

    fn set_content_view(self, view: Object) {
        unsafe { msg_send![self, setContentView: view] }
    }
}
