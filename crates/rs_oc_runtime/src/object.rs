use crate::Encode;

use super::object_getClass;
use super::{Class, PrivateMarker};
#[repr(C)]
pub struct Object {
    _priv: PrivateMarker,
}

/// 类实例化后的对象
impl Object {
    /// 调用 oc runtime 中的 object_getClass 方法来获取类自身
    pub fn class(&self) -> &Class {
        unsafe { &*object_getClass(self) }
    }

    /// 获取成员变量，这是一个 unsafe 方法，因为它不会检查类型是否一致，调用者需要自己保证类型一致
    pub unsafe fn get_ivar<T>(&self, name: &str) -> &T
    where
        T: Encode,
    {
        let offset = {
            let cls = self.class();
            match cls.instance_variable(name) {
                Some(ivar) => {
                    // 判断类型是否一致
                    assert!(ivar.type_encoding() == &T::ENCODING);
                    ivar.offset()
                }
                None => panic!("{} not found", name),
            }
        };
        let self_ptr: *const Object = self;
        let ptr = (self_ptr as *const u8).offset(offset) as *const T;
        &*ptr
    }

    /// 与 get_ivar 类似，但是返回的是可变引用
    pub unsafe fn get_mut_ivar<T>(&mut self, name: &str) -> &mut T
    where
        T: Encode,
    {
        let offset = {
            let cls = self.class();
            match cls.instance_variable(name) {
                Some(ivar) => {
                    // 判断类型是否一致
                    assert!(ivar.type_encoding() == &T::ENCODING);
                    ivar.offset()
                }
                None => panic!("{} not found", name),
            }
        };
        let self_ptr: *mut Object = self;
        let ptr = (self_ptr as *mut u8).offset(offset) as *mut T;
        &mut *ptr
    }

    /// 设置成员变量，也是 unsafe 的，同 get_ivar 方法。需要自己保证类型一致
    pub unsafe fn set_ivar<T>(&mut self, name: &str, value: T)
    where
        T: Encode,
    {
        *self.get_mut_ivar::<T>(name) = value;
    }
}
