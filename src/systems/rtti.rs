use crate::raw::root::RED4ext as red;
use crate::{CName, Class, Enum, Type};

#[repr(transparent)]
pub struct CRTTISystem(*mut red::CRTTISystem);

impl CRTTISystem {
    pub fn get() -> Self {
        unsafe { Self(red::CRTTISystem_Get()) }
    }

    #[inline]
    pub fn get_class(&self, name: CName) -> Option<&Class> {
        let class = unsafe { (self.vft().base.get_class)(&(*self.0)._base, name.0) };
        if class.is_null() {
            return None;
        }
        Some(unsafe { &*class.cast::<Class>() })
    }

    #[inline]
    pub fn get_type(&self, name: CName) -> Option<&Type> {
        let ty = unsafe { (self.vft().base.get_type)(&(*self.0)._base, name.0) };
        if ty.is_null() {
            return None;
        }
        Some(unsafe { &*ty.cast::<Type>() })
    }

    #[inline]
    pub fn get_enum(&self, name: CName) -> Option<&Enum> {
        let ty = unsafe { (self.vft().base.get_enum)(&(*self.0)._base, name.0) };
        if ty.is_null() {
            return None;
        }
        Some(unsafe { &*ty.cast::<Enum>() })
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
        name: red::CName,
    ) -> *const red::CBaseRTTIType,
    pub get_type_by_async_id: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        async_id: u32,
    ) -> *const red::CBaseRTTIType,
    pub get_class: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: red::CName,
    ) -> *const red::CClass,
    pub get_enum: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: red::CName,
    ) -> *const red::CEnum,
    pub get_bit_field: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: *const red::CName,
    ) -> *const red::CBitfield,
    sub_28: unsafe extern "fastcall" fn(this: *const red::IRTTISystem),
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
    pub get_derived_classes: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        base_class: *const red::CClass,
        out: *mut red::DynArray<*const red::CClass>,
    ),
    pub register_type: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        ty: *const red::CBaseRTTIType,
        async_id: u32,
    ),
    sub_88: unsafe extern "fastcall" fn(this: *const red::IRTTISystem),
    sub_90: unsafe extern "fastcall" fn(this: *const red::IRTTISystem),
    pub unregister_type:
        unsafe extern "fastcall" fn(this: *const red::IRTTISystem, ty: *const red::CBaseRTTIType),
    pub register_function: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        function: *const red::CGlobalFunction,
    ),
    pub unregister_function: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        function: *const red::CGlobalFunction,
    ),
    sub_b0: unsafe extern "fastcall" fn(this: *const red::IRTTISystem),
    sub_b8: unsafe extern "fastcall" fn(this: *const red::IRTTISystem),
    // FIXME: crashes when used, signature is probably wrong
    add_register_callback: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        function: unsafe extern "C" fn() -> (),
    ),
    // FIXME: crashes when used, signature is probably wrong
    add_post_register_callback: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        function: unsafe extern "C" fn() -> (),
    ),
    sub_d0: unsafe extern "fastcall" fn(this: *const red::IRTTISystem),
    sub_d8: unsafe extern "fastcall" fn(this: *const red::IRTTISystem),
    pub create_scripted_class: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: red::CName,
        flags: red::CClass_Flags,
        parent: *const red::CClass,
    ),
    // FIXME: signature is wrong, but how to represent name and value of enumerator ?
    // https://github.com/WopsS/RED4ext.SDK/blob/124984353556f7b343041b810040062fbaa96196/include/RED4ext/RTTISystem.hpp#L50
    pub create_scripted_enum: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: red::CName,
        size: i8,
        variants: *mut red::DynArray<u64>,
    ),
    // FIXME: signature is wrong, but how to represent name and bit ?
    // https://github.com/WopsS/RED4ext.SDK/blob/124984353556f7b343041b810040062fbaa96196/include/RED4ext/RTTISystem.hpp#L54
    pub create_scripted_bitfield: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: red::CName,
        bits: *mut red::DynArray<u64>,
    ),
    initialize_script_runtime: unsafe extern "fastcall" fn(this: *const red::IRTTISystem),
    pub register_script_name: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        native_name: red::CName,
        script_name: red::CName,
    ),
    pub get_class_by_script_name: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: red::CName,
    ) -> *const red::CClass,
    pub get_enum_by_script_name: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: red::CName,
    ) -> *const red::CEnum,
    pub convert_native_to_script_name:
        unsafe extern "fastcall" fn(this: *const red::IRTTISystem, name: red::CName) -> red::CName,
    pub convert_script_to_native_name:
        unsafe extern "fastcall" fn(this: *const red::IRTTISystem, name: red::CName) -> red::CName,
}

#[repr(transparent)]
pub struct RTTIRegistrator;
impl RTTIRegistrator {
    pub fn add(
        register: Option<unsafe extern "C" fn()>,
        post_register: Option<unsafe extern "C" fn()>,
    ) {
        unsafe { red::RTTIRegistrator::Add(register, post_register, false) };
    }
}
