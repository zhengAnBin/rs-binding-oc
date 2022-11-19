use rs_oc_basic::{class, msg_send, sel, sel_impl, ClassDecl, ObjcClass, ObjcObject, Object, Sel};
use std::os::raw::c_void;

extern "C" fn get_instance_ptr(this: &ObjcObject, _sel: Sel) -> *const c_void {
    unsafe { *this.get_ivar("_instance_ptr") }
}

extern "C" fn set_instance_ptr(this: &mut ObjcObject, _sel: Sel, instance_ptr: *const c_void) {
    unsafe { this.set_ivar("_instance_ptr", instance_ptr) };
}

extern "C" fn call_ptr<Func>(this: &ObjcObject, _sel: Sel, controller: Object, message: Object)
where
    Func: FnMut(Object, Object),
{
    unsafe {
        let instance_ptr: *mut c_void = msg_send![this, instancePtr];
        let instance_ptr = instance_ptr as *mut Func;
        (*instance_ptr)(controller, message);
    }
}

pub unsafe fn make_handler<Func>(name: &str, func: *mut Func) -> Object
where
    Func: FnMut(Object, Object),
{
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new(name, superclass).unwrap();

    decl.add_ivar::<*const c_void>("_instance_ptr");
    decl.add_method::<extern "C" fn(&ObjcObject, Sel) -> *const c_void>(
        sel!(instancePtr),
        get_instance_ptr,
    );
    decl.add_method::<extern "C" fn(&mut ObjcObject, Sel, *const c_void)>(
        sel!(setInstancePtr:),
        set_instance_ptr,
    );
    decl.add_method::<extern "C" fn(&ObjcObject, Sel, Object, Object)>(
        sel!(userContentController:didReceiveScriptMessage:),
        call_ptr::<Func>,
    );
    decl.register();

    let class = ObjcClass::get(name).unwrap();
    let instance: Object = msg_send![class, alloc];
    let instance: Object = msg_send![instance, init];
    instance.set_instance_ptr(func as *mut c_void);

    instance
}

pub trait WKScriptMessageHandlerBridge: Sized {
    unsafe fn set_instance_ptr(self, _: *mut c_void);
}

impl WKScriptMessageHandlerBridge for Object {
    unsafe fn set_instance_ptr(self, instance_ptr: *mut c_void) {
        msg_send![self, setInstancePtr: instance_ptr]
    }
}
