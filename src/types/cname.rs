use std::ffi::{self, CStr};
use std::fmt;
use std::hash::Hash;

use crate::fnv1a64;
use crate::raw::root::RED4ext as red;

#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct CName(red::CName);

impl CName {
    #[inline]
    pub const fn new(name: &str) -> Self {
        Self(red::CName {
            hash: fnv1a64(name),
        })
    }

    #[inline]
    pub const fn undefined() -> Self {
        Self(red::CName { hash: 0 })
    }

    pub(super) fn from_raw(raw: red::CName) -> Self {
        Self(raw)
    }

    pub(super) fn to_raw(self) -> red::CName {
        self.0
    }

    pub fn as_str(&self) -> &'static str {
        unsafe { ffi::CStr::from_ptr(self.0.ToString()) }
            .to_str()
            .unwrap()
    }
}

impl From<u64> for CName {
    fn from(hash: u64) -> Self {
        Self(red::CName { hash })
    }
}

impl std::fmt::Display for CName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            if self.0.hash == 0 {
                "None"
            } else {
                self.as_str()
            }
        )
    }
}

impl PartialEq for CName {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.hash == other.0.hash
    }
}

impl Eq for CName {}

impl PartialOrd for CName {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CName {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.hash.cmp(&other.0.hash)
    }
}

impl Hash for CName {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash.hash(state)
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct CNamePool(red::CNamePool);

impl CNamePool {
    pub fn add_cstr(str: &CStr) -> CName {
        unsafe {
            let add_cstr = crate::fn_from_hash!(
                CNamePool_AddCstr,
                unsafe extern "C" fn(&mut CName, *const i8)
            );
            let mut cname = CName::default();
            add_cstr(&mut cname, str.as_ptr());
            cname
        }
    }
}
