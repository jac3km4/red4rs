use crate::raw::root::RED4ext as red;
use crate::{Array, Bitfield, CName, Class, Enum, Function, Type};

#[repr(transparent)]
pub struct RttiSystem(red::CRTTISystem);

impl RttiSystem {
    pub fn get() -> Self {
        unsafe { Self(std::ptr::read(red::CRTTISystem_Get())) }
    }

    #[inline]
    pub fn get_class(&self, name: CName) -> Option<&Class> {
        let ty = unsafe { (self.vft().base.get_class)(&self.0._base, name.0) };
        unsafe { ty.cast::<Class>().as_ref() }
    }

    #[inline]
    pub fn get_type(&self, name: CName) -> Option<&Type> {
        let ty = unsafe { (self.vft().base.get_type)(&self.0._base, name.0) };
        unsafe { ty.cast::<Type>().as_ref() }
    }

    #[inline]
    pub fn get_enum(&self, name: CName) -> Option<&Enum> {
        let ty = unsafe { (self.vft().base.get_enum)(&self.0._base, name.0) };
        unsafe { ty.cast::<Enum>().as_ref() }
    }

    #[inline]
    pub fn get_bitfield(&self, name: CName) -> Option<&Bitfield> {
        let ty = unsafe { (self.vft().base.get_bitfield)(&self.0._base, name.0) };
        unsafe { ty.cast::<Bitfield>().as_ref() }
    }

    #[inline]
    pub fn get_function(&self, name: CName) -> Option<&Function> {
        let ty = unsafe { (self.vft().base.get_function)(&self.0._base, name.0) };
        unsafe { ty.cast::<Function>().as_ref() }
    }

    #[inline]
    pub fn get_native_types(&self) -> Vec<Type> {
        let mut out = Array::default();
        unsafe { (self.vft().base.get_native_types)(&self.0._base, &mut out.0 as *mut _) };
        out.as_ref()
            .iter()
            .map(|x| Type(unsafe { std::ptr::read(*x) }))
            .collect()
    }

    #[inline]
    pub fn get_enums(&self) -> Vec<Enum> {
        let mut out = Array::default();
        unsafe { (self.vft().base.get_enums)(&self.0._base, &mut out.0 as *mut _) };
        out.as_ref()
            .iter()
            .map(|x| Enum(unsafe { std::ptr::read(*x) }))
            .collect()
    }

    #[inline]
    pub fn get_bitfields(&self, scripted_only: bool) -> Vec<Bitfield> {
        let mut out = Array::default();
        unsafe {
            (self.vft().base.get_bitfields)(&self.0._base, &mut out.0 as *mut _, scripted_only)
        };
        out.as_ref()
            .iter()
            .map(|x| Bitfield(unsafe { std::ptr::read(*x) }))
            .collect()
    }

    #[inline]
    pub fn get_global_functions(&self) -> Vec<Function> {
        let mut out = Array::default();
        unsafe { (self.vft().base.get_global_functions)(&self.0._base, &mut out.0 as *mut _) };
        out.as_ref()
            .iter()
            .map(|x| Function(unsafe { std::ptr::read(*x) }))
            .collect()
    }

    #[inline]
    pub fn get_class_functions(&self) -> Vec<Function> {
        let mut out = Array::default();
        unsafe { (self.vft().base.get_class_functions)(&self.0._base, &mut out.0 as *mut _) };
        out.as_ref()
            .iter()
            .map(|x| Function(unsafe { std::ptr::read(*x) }))
            .collect()
    }

    /// retrieve base class and its inheritors, optionally including abstract classes.
    #[inline]
    pub fn get_classes(&self, base: &Class, include_abstract: bool) -> Vec<Class> {
        let mut out = Array::default();
        unsafe {
            (self.vft().base.get_classes)(
                &self.0._base,
                &base.0,
                &mut out.0 as *mut _,
                None,
                include_abstract,
            )
        };
        out.as_ref()
            .iter()
            .map(|x| Class(unsafe { std::ptr::read(*x) }))
            .collect()
    }

    /// retrieve derived classes, omitting base in the output.
    #[inline]
    pub fn get_derived_classes(&self, base: &Class) -> Vec<Class> {
        let mut out = Array::default();
        unsafe {
            (self.vft().base.get_derived_classes)(&self.0._base, &base.0, &mut out.0 as *mut _)
        };
        out.as_ref()
            .iter()
            .map(|x| Class(unsafe { std::ptr::read(*x) }))
            .collect()
    }

    #[inline]
    pub fn get_class_by_script_name(&self, name: CName) -> Option<&Class> {
        let ty = unsafe { (self.vft().base.get_class_by_script_name)(&self.0._base, name.0) };
        unsafe { ty.cast::<Class>().as_ref() }
    }

    #[inline]
    pub fn get_enum_by_script_name(&self, name: CName) -> Option<&Enum> {
        let ty = unsafe { (self.vft().base.get_enum_by_script_name)(&self.0._base, name.0) };
        unsafe { ty.cast::<Enum>().as_ref() }
    }

    #[inline]
    fn vft(&self) -> &RTTISystemVft {
        unsafe { &*(self.0._base.vtable_ as *const RTTISystemVft) }
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
    pub get_bitfield: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: red::CName,
    ) -> *const red::CBitfield,
    sub_28: unsafe extern "fastcall" fn(this: *const red::IRTTISystem),
    pub get_function: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        name: red::CName,
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
    pub get_bitfields: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        out: *mut red::DynArray<*const red::CBitfield>,
        scripted_only: bool,
    ),
    pub get_classes: unsafe extern "fastcall" fn(
        this: *const red::IRTTISystem,
        base_class: *const red::CClass,
        out: *mut red::DynArray<*const red::CClass>,
        filter: Option<unsafe extern "C" fn(*const Class) -> bool>,
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
    // FIXME: crashes when used, signature is probably wrong
    convert_native_to_script_name:
        unsafe extern "fastcall" fn(this: *const red::IRTTISystem, name: red::CName) -> red::CName,
    // FIXME: crashes when used, signature is probably wrong
    convert_script_to_native_name:
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
