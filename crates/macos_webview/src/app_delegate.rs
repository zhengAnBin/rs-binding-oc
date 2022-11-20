use rs_oc_appKit::NSApplicationDelegate;
use rs_oc_basic::{class, sel, sel_impl, ClassDecl, ObjcClass, ObjcObject, Object, Sel};

pub(crate) static RS_TRAIT_PTR: &str = "rsTraitPtr";

fn get_trait<T>(this: &ObjcObject) -> &T {
    unsafe {
        let app_ptr: usize = *this.get_ivar(RS_TRAIT_PTR);
        let app = app_ptr as *const T;
        &*app
    }
}

extern "C" fn application_will_finish_launching<T: NSApplicationDelegate>(
    this: &ObjcObject,
    _: Sel,
    _: Object,
) {
    get_trait::<T>(this).application_will_finish_launching();
}

pub fn register_ns_app_lication_delegate<T: NSApplicationDelegate>() -> *const ObjcClass {
    unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("RSAppDelegate", superclass).unwrap();

        // rust trait ptr
        decl.add_ivar::<usize>(RS_TRAIT_PTR);

        decl.add_method(
            sel!(applicationWillFinishLaunching:),
            application_will_finish_launching::<T> as extern "C" fn(&ObjcObject, _, _),
        );

        return decl.register();
    }
}
