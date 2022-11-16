use rs_oc_appKit::{
    NSApplication, NSApplicationActivationOptions, NSApplicationActivationPolicy,
    NSBackingStoreType, NSRunningApplication, NSWindow, NSWindowStyleMask,
};
use rs_oc_basic::{NIL, NO};
use rs_oc_core_graphics::{CGPoint, CGRect, CGSize};
use rs_oc_foundation::{NSAutoreleasePool, NSString};

fn main() {
    let _pool = NSAutoreleasePool::new(NIL);

    let app = NSApplication::shared_application(NIL);
    app.set_activation_policy(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);

    let cg_rect = CGRect::new(&CGPoint::new(0., 0.), &CGSize::new(100., 100.));
    let window = NSWindow::alloc(NIL)
        .init_with_content_rect_stylemask_backing_defer(
            cg_rect,
            NSWindowStyleMask::NSTitledWindowMask,
            NSBackingStoreType::NSBackingStoreBuffered,
            NO,
        )
        .autorelease();

    window.cascade_top_left_from_point(CGPoint { x: 20., y: 20. });
    window.center();
    let title = NSString::alloc(NIL).init_with_bytes("123");
    window.set_title(title);
    window.make_key_and_order_front(NIL);

    let current_app = NSRunningApplication::current_application(NIL);
    current_app.activate_with_options(
        NSApplicationActivationOptions::NSApplicationActivateIgnoringOtherApps,
    );

    app.run();
}
