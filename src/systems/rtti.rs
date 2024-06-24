use crate::raw::root::RED4ext as red;
use crate::{CName, Class};

#[repr(transparent)]
pub struct CRTTISystem(*mut red::CRTTISystem);

impl CRTTISystem {
    pub fn get() -> Self {
        unsafe { Self(red::CRTTISystem_Get()) }
    }

    #[inline]
    pub fn get_class(&self, name: CName) -> Option<&Class> {
        let class = unsafe { (self.vft().base.get_class)(&(*self.0)._base, &name.0 as *const _) };
        if class.is_null() {
            return None;
        }
        Some(unsafe { &*class.cast::<Class>() })
    }

    #[inline]
    fn vft(&self) -> &RTTISystemVft {
        unsafe { &*((*self.0)._base.vtable_ as *const RTTISystemVft) }
    }
}

#[repr(C)]
struct RTTISystemVft {
    base: IRTTISystemVft,
}

#[repr(C)]
struct IRTTISystemVft {
    pub get_type: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: *const red::CName,
    ) -> *const red::CBaseRTTIType,
    pub get_type_by_async_id: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        id: u32,
    ) -> *const red::CBaseRTTIType,
    pub get_class: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: *const red::CName,
    ) -> *const red::CClass,
    pub get_enum: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: *const red::CName,
    ) -> *const red::CEnum,
    pub get_bit_field: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: *const red::CName,
    ) -> *const red::CBitfield,
    sub_28: unsafe extern "fastcall" fn(this: *const red::IRTTISystem) -> VoidPtr,
    pub get_function: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: *const red::CName,
    ) -> *const red::CBaseFunction,
    sub_38: unsafe extern "fastcall" fn(this: *const red::IRTTISystem),
    pub get_native_types: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        out: *mut red::DynArray<*const red::CBaseRTTIType>,
    ),
    pub get_global_functions: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        out: *mut red::DynArray<*const red::CBaseFunction>,
    ),
    sub_50: unsafe extern "fastcall" fn(this: *const red::IRTTISystem),
    pub get_class_functions: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        out: *mut red::DynArray<*const red::CBaseFunction>,
    ),
    pub get_enums: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        out: *mut red::DynArray<*const red::CEnum>,
    ),
    pub get_bit_fields: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        out: *mut red::DynArray<*const red::CBitfield>,
        scripted_only: bool,
    ),
    pub get_classes: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        class: *const red::CClass,
        out: *mut red::DynArray<*const red::CClass>,
        filter: *const fn(*const red::CClass) -> bool,
        include_abstract: bool,
    ),
}
