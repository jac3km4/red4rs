use std::pin::Pin;

use crate::raw::root::RED4ext as red;
use crate::{CName, Class};

#[repr(C)]
#[allow(non_snake_case)]
struct RttiVft {
    pub IRTTISystem_GetType: unsafe extern "fastcall" fn(
        this: &red::IRTTISystem,
        name: *const red::CName,
    ) -> *const red::CBaseRTTIType,
    pub IRTTISystem_GetTypeByAsyncId:
        unsafe extern "fastcall" fn(this: &red::IRTTISystem, id: u32) -> *const red::CBaseRTTIType,
    pub IRTTISystem_GetClass: unsafe extern "fastcall" fn(
        this: &red::IRTTISystem,
        this: *const red::CName,
    ) -> *const red::CClass,
}

#[allow(dead_code)]
pub struct Rtti<'a> {
    inner: Pin<&'a mut red::CRTTISystem>,
}

impl<'a> Rtti<'a> {
    #[inline]
    pub fn get() -> Self {
        Self {
            inner: unsafe { Pin::new_unchecked(&mut *red::CRTTISystem::Get()) },
        }
    }

    pub fn get_class(&self, name: CName) -> Option<&Class> {
        let class =
            unsafe { (self.vft().IRTTISystem_GetClass)(&self.inner._base, &name.0 as *const _) };
        if class.is_null() {
            return None;
        }
        Some(unsafe { &*class.cast::<Class>() })
    }

    #[inline]
    fn vft(&self) -> &RttiVft {
        unsafe { &*(self.inner._base.vtable_ as *const RttiVft) }
    }
}
