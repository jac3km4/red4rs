use std::ffi::CStr;
use std::{iter, mem};

use super::{
    Array, CName, CNamePool, IAllocator, Native, PoolRef, PoolableOps, ScriptClass, ScriptRefAny,
    StackFrame,
};
use crate::raw::root::RED4ext as red;
use crate::VoidPtr;

pub type FunctionHandler<R> = extern "C" fn(Option<&IScriptable>, &mut StackFrame, &mut R, i64);

#[derive(Debug)]
#[repr(transparent)]
pub struct Type(red::CBaseRTTIType);

impl Type {
    #[inline]
    pub fn name(&self) -> CName {
        // calling Type with unk8 == 0 crashes the game
        if self.0.unk8 == 0 {
            return CName::undefined();
        }
        CName::from_raw(unsafe { (self.vft().tail.CBaseRTTIType_GetName)(&self.0) })
    }

    #[inline]
    pub fn size(&self) -> u32 {
        unsafe { (self.vft().tail.CBaseRTTIType_GetSize)(&self.0) }
    }

    #[inline]
    pub fn alignment(&self) -> u32 {
        unsafe { (self.vft().tail.CBaseRTTIType_GetAlignment)(&self.0) }
    }

    #[inline]
    pub fn kind(&self) -> Kind {
        unsafe { mem::transmute((self.vft().tail.CBaseRTTIType_GetType)(&self.0)) }
    }

    #[inline]
    pub fn as_class(&self) -> Option<&Class> {
        if self.kind().is_class() {
            Some(unsafe { mem::transmute::<&red::CBaseRTTIType, &Class>(&self.0) })
        } else {
            None
        }
    }

    #[inline]
    pub fn as_array(&self) -> Option<&ArrayType> {
        if self.kind().is_array() {
            Some(unsafe { mem::transmute::<&red::CBaseRTTIType, &ArrayType>(&self.0) })
        } else {
            None
        }
    }

    pub unsafe fn to_string(&self, value: ValuePtr) -> String {
        let mut str = String::new();
        unsafe {
            (self.vft().tail.CBaseRTTIType_ToString)(
                &self.0,
                value.0,
                &mut str as *mut _ as *mut red::CString,
            )
        };
        str
    }

    #[inline]
    fn vft(&self) -> &TypeVft {
        unsafe { &*(self.0.vtable_.cast::<TypeVft>()) }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Kind {
    Name = red::ERTTIType::Name,
    Fundamental = red::ERTTIType::Fundamental,
    Class = red::ERTTIType::Class,
    Array = red::ERTTIType::Array,
    Simple = red::ERTTIType::Simple,
    Enum = red::ERTTIType::Enum,
    StaticArray = red::ERTTIType::StaticArray,
    NativeArray = red::ERTTIType::NativeArray,
    Pointer = red::ERTTIType::Pointer,
    Handle = red::ERTTIType::Handle,
    WeakHandle = red::ERTTIType::WeakHandle,
    ResourceReference = red::ERTTIType::ResourceReference,
    ResourceAsyncReference = red::ERTTIType::ResourceAsyncReference,
    BitField = red::ERTTIType::BitField,
    LegacySingleChannelCurve = red::ERTTIType::LegacySingleChannelCurve,
    ScriptReference = red::ERTTIType::ScriptReference,
    FixedArray = red::ERTTIType::FixedArray,
}

impl Kind {
    #[inline]
    pub fn is_pointer(self) -> bool {
        matches!(self, Self::Pointer | Self::Handle | Self::WeakHandle)
    }

    #[inline]
    pub fn is_class(self) -> bool {
        self == Self::Class
    }

    #[inline]
    pub fn is_array(self) -> bool {
        matches!(
            self,
            Self::Array | Self::StaticArray | Self::NativeArray | Self::FixedArray
        )
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Class(red::CClass);

impl Class {
    #[inline]
    pub fn name(&self) -> CName {
        CName::from_raw(self.0.name)
    }

    #[inline]
    pub fn properties(&self) -> &Array<&Property> {
        unsafe { mem::transmute(&self.0.props) }
    }

    #[inline]
    pub fn base(&self) -> Option<&Class> {
        unsafe { (self.0.parent as *const Class).as_ref() }
    }

    #[inline]
    pub fn base_iter(&self) -> impl Iterator<Item = &Class> {
        iter::successors(self.base(), |class| class.base())
    }

    pub fn all_properties(&self) -> impl Iterator<Item = &Property> {
        iter::once(self)
            .chain(self.base_iter())
            .flat_map(Class::properties)
            .copied()
    }

    #[inline]
    pub fn instantiate(&self) -> ValueContainer {
        ValueContainer(unsafe { self.0.CreateInstance(true) })
    }

    #[inline]
    pub fn as_type(&self) -> &Type {
        unsafe { &*(self as *const _ as *const Type) }
    }

    #[inline]
    pub fn as_type_mut(&mut self) -> &mut Type {
        unsafe { &mut *(self as *mut _ as *mut Type) }
    }
}

impl Drop for Class {
    #[inline]
    fn drop(&mut self) {
        let t = self.as_type_mut();
        unsafe { (t.vft().destroy)(t) };
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Function(red::CBaseFunction);

impl Function {
    #[inline]
    pub fn name(&self) -> CName {
        CName::from_raw(self.0.fullName)
    }

    #[inline]
    pub fn parent(&self) -> Option<&Class> {
        unsafe { (self.vft().get_parent)(self).as_ref() }
    }

    #[inline]
    pub fn locals(&self) -> &Array<&Property> {
        unsafe { mem::transmute(&self.0.localVars) }
    }

    #[inline]
    pub fn params(&self) -> &Array<&Property> {
        unsafe { mem::transmute(&self.0.params) }
    }

    #[inline]
    pub fn is_static(&self) -> bool {
        self.0.flags.isStatic() != 0
    }

    #[inline]
    pub fn add_param(&mut self, typ: CName, name: &CStr, is_out: bool, is_optional: bool) -> bool {
        unsafe {
            self.0
                .AddParam(typ.to_raw(), name.as_ptr(), is_out, is_optional)
        }
    }

    #[inline]
    pub fn set_return_type(&mut self, typ: CName) {
        unsafe { self.0.SetReturnType(typ.to_raw()) };
    }

    #[inline]
    pub fn set_is_native(&mut self, is_native: bool) {
        self.0.flags.set_isNative(is_native as u32)
    }

    #[inline]
    pub fn set_is_final(&mut self, is_final: bool) {
        self.0.flags.set_isFinal(is_final as u32)
    }

    #[inline]
    pub fn set_is_static(&mut self, is_static: bool) {
        self.0.flags.set_isStatic(is_static as u32)
    }

    #[inline]
    fn vft(&self) -> &FunctionVft {
        unsafe { &*(self.0._base.vtable_.cast::<FunctionVft>()) }
    }
}

impl Drop for Function {
    #[inline]
    fn drop(&mut self) {
        unsafe { (self.vft().destruct)(self) };
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct GlobalFunction(red::CGlobalFunction);

impl GlobalFunction {
    pub fn new<R>(
        full_name: &CStr,
        short_name: &CStr,
        handler: FunctionHandler<R>,
    ) -> PoolRef<Self> {
        let mut func = GlobalFunction::alloc().expect("should allocate a GlobalFunction");
        let full_name = CNamePool::add_cstr(full_name);
        let short_name = CNamePool::add_cstr(short_name);

        Self::ctor(func.as_mut_ptr(), full_name, short_name, handler as _);
        unsafe { func.assume_init() }
    }

    fn ctor(ptr: *mut Self, full_name: CName, short_name: CName, handler: VoidPtr) {
        unsafe {
            let ctor = crate::fn_from_hash!(
                CGlobalFunction_ctor,
                unsafe extern "C" fn(*mut GlobalFunction, CName, CName, VoidPtr)
            );
            ctor(ptr, full_name, short_name, handler);
        };
    }

    #[inline]
    pub fn as_function(&self) -> &Function {
        unsafe { &*(self as *const _ as *const Function) }
    }

    #[inline]
    pub fn as_function_mut(&mut self) -> &mut Function {
        unsafe { &mut *(self as *mut _ as *mut Function) }
    }
}

impl Drop for GlobalFunction {
    #[inline]
    fn drop(&mut self) {
        let f = self.as_function_mut();
        unsafe { (f.vft().destruct)(f) };
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Property(red::CProperty);

impl Property {
    #[inline]
    pub fn name(&self) -> CName {
        CName::from_raw(self.0.name)
    }

    #[inline]
    pub fn type_(&self) -> &'static Type {
        unsafe { &*(self.0.type_ as *const Type) }
    }

    #[inline]
    pub unsafe fn value(&self, container: ValueContainer) -> ValuePtr {
        unsafe { ValuePtr(container.0.byte_add(self.0.valueOffset as usize)) }
    }

    #[inline]
    pub fn is_in_value_holder(&self) -> bool {
        self.0.flags.inValueHolder() != 0
    }

    #[inline]
    pub fn is_scripted(&self) -> bool {
        self.0.flags.isScripted() != 0
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ArrayType(red::CRTTIBaseArrayType);

impl ArrayType {
    #[inline]
    pub fn inner_type(&self) -> &'static Type {
        unsafe { &*(self.vft().get_inner_type)(self) }
    }

    #[inline]
    pub unsafe fn length(&self, val: ValuePtr) -> u32 {
        unsafe { (self.vft().get_length)(self, val) }
    }

    #[inline]
    pub unsafe fn element(&self, val: ValuePtr, index: u32) -> ValuePtr {
        unsafe { (self.vft().get_element)(self, val, index) }
    }

    #[inline]
    pub fn as_type(&self) -> &Type {
        unsafe { &*(self as *const _ as *const Type) }
    }

    #[inline]
    pub fn as_type_mut(&mut self) -> &mut Type {
        unsafe { &mut *(self as *mut _ as *mut Type) }
    }

    #[inline]
    fn vft(&self) -> &ArrayTypeVft {
        unsafe { &*(self.0._base.vtable_ as *const ArrayTypeVft) }
    }
}

impl Drop for ArrayType {
    #[inline]
    fn drop(&mut self) {
        let t = self.as_type_mut();
        unsafe { (t.vft().destroy)(t) };
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Enum(red::CEnum);

impl Enum {
    #[inline]
    pub fn name(&self) -> CName {
        CName::from_raw(self.0.name)
    }

    #[inline]
    pub fn variant_names(&self) -> &Array<CName> {
        unsafe { mem::transmute(&self.0.aliasList) }
    }

    #[inline]
    pub fn as_type(&self) -> &Type {
        unsafe { &*(self as *const _ as *const Type) }
    }

    #[inline]
    pub fn as_type_mut(&mut self) -> &mut Type {
        unsafe { &mut *(self as *mut _ as *mut Type) }
    }
}

impl Drop for Enum {
    #[inline]
    fn drop(&mut self) {
        let t = self.as_type_mut();
        unsafe { (t.vft().destroy)(t) };
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Bitfield(red::CBitfield);

impl Bitfield {
    pub fn name(&self) -> CName {
        CName::from_raw(self.0.name)
    }

    pub fn fields(&self) -> &[CName; 64] {
        unsafe { mem::transmute(&self.0.bitNames) }
    }

    pub fn as_type(&self) -> &Type {
        unsafe { &*(self as *const _ as *const Type) }
    }

    pub fn as_type_mut(&mut self) -> &mut Type {
        unsafe { &mut *(self as *mut _ as *mut Type) }
    }
}

impl Drop for Bitfield {
    #[inline]
    fn drop(&mut self) {
        let t = self.as_type_mut();
        unsafe { (t.vft().destroy)(t) };
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct IScriptable(red::IScriptable);

impl IScriptable {
    pub fn class(&self) -> &'static Class {
        unsafe {
            &*(((*self.0._base.vtable_).ISerializable_GetType)(
                (&self.0._base) as *const _ as *mut red::ISerializable,
            ) as *const Class)
        }
    }

    #[inline]
    pub fn fields(&self) -> ValueContainer {
        ValueContainer(unsafe {
            red::IScriptable_GetValueHolder(&self.0 as *const _ as *mut red::IScriptable)
        })
    }
}

impl AsRef<IScriptable> for IScriptable {
    #[inline]
    fn as_ref(&self) -> &IScriptable {
        self
    }
}

impl AsMut<IScriptable> for IScriptable {
    #[inline]
    fn as_mut(&mut self) -> &mut IScriptable {
        self
    }
}

unsafe impl ScriptClass for IScriptable {
    type Kind = Native;

    const CLASS_NAME: &'static str = "IScriptable";
}

#[derive(Debug, Clone, Copy)]
pub struct ValueContainer(VoidPtr);

impl ValueContainer {
    #[inline]
    pub(super) fn new(ptr: VoidPtr) -> Self {
        Self(ptr)
    }

    #[inline]
    pub(super) fn as_ptr(&self) -> VoidPtr {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ValuePtr(VoidPtr);

impl ValuePtr {
    pub(super) fn new(ptr: VoidPtr) -> Self {
        Self(ptr)
    }

    pub unsafe fn unwrap_ref(&self) -> Option<&IScriptable> {
        let ptr = self.0 as *mut red::SharedPtrBase<red::IScriptable>;
        let inst = (*ptr).instance;
        let rc = (*ptr).refCount;
        if inst.is_null() || rc.is_null() || (*rc).strongRefs == 0 {
            return None;
        };
        Some(&*(inst as *const IScriptable))
    }

    pub unsafe fn unwrap_script_ref(&self) -> Option<ValuePtr> {
        let ptr = &*(self.0 as *mut ScriptRefAny);
        if !ptr.is_defined() {
            return None;
        };
        Some(ptr.value())
    }

    #[inline]
    pub unsafe fn to_container(&self) -> ValueContainer {
        ValueContainer(self.0)
    }
}

#[repr(C)]
struct TypeVft {
    destroy: unsafe extern "fastcall" fn(this: *mut Type),
    tail: red::CBaseRTTIType__bindgen_vtable,
}

#[repr(C)]
struct ArrayTypeVft {
    base: TypeVft,
    get_inner_type: unsafe extern "fastcall" fn(this: *const ArrayType) -> *const Type,
    sub_c8: unsafe extern "fastcall" fn(this: *const ArrayType) -> bool,
    get_length: unsafe extern "fastcall" fn(this: *const ArrayType, val: ValuePtr) -> u32,
    get_max_length: unsafe extern "fastcall" fn(this: *const ArrayType) -> u32,
    get_element:
        unsafe extern "fastcall" fn(this: *const ArrayType, val: ValuePtr, index: u32) -> ValuePtr,
}

#[repr(C)]
struct FunctionVft {
    get_allocator: unsafe extern "fastcall" fn(this: &Function) -> *mut IAllocator,
    destruct: unsafe extern "fastcall" fn(this: &mut Function),
    get_parent: unsafe extern "fastcall" fn(this: &Function) -> *mut Class,
}
