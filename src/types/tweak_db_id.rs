use std::fmt::Debug;
use std::hash::Hash;

use byteorder::{BigEndian, ByteOrder};
use const_crc32::{crc32, crc32_seed};

use crate::raw::root::RED4ext as red;

#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct TweakDbId(red::TweakDBID);

impl Debug for TweakDbId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TweakDbId")
            .field(&unsafe { self.0.__bindgen_anon_1.value })
            .finish()
    }
}

impl PartialEq for TweakDbId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.0.__bindgen_anon_1.value == other.0.__bindgen_anon_1.value }
    }
}

impl Eq for TweakDbId {}

impl PartialOrd for TweakDbId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for TweakDbId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&unsafe { self.0.__bindgen_anon_1.value }).cmp(&unsafe { other.0.__bindgen_anon_1.value })
    }
}

impl From<u64> for TweakDbId {
    fn from(value: u64) -> Self {
        Self(red::TweakDBID {
            __bindgen_anon_1: red::TweakDBID__bindgen_ty_1 { value },
        })
    }
}

impl From<TweakDbId> for u64 {
    fn from(value: TweakDbId) -> Self {
        unsafe { value.0.__bindgen_anon_1.value }
    }
}

impl TweakDbId {
    #[inline]
    pub const fn new(str: &str) -> Self {
        assert!(str.len() <= u8::MAX as usize);
        Self(red::TweakDBID {
            __bindgen_anon_1: red::TweakDBID__bindgen_ty_1 {
                name: red::TweakDBID__bindgen_ty_1__bindgen_ty_1 {
                    hash: crc32(str.as_bytes()),
                    length: str.len() as u8,
                    tdbOffsetBE: [0, 0, 0],
                },
            },
        })
    }

    #[inline]
    pub const fn new_from_base(base: TweakDbId, str: &str) -> Self {
        let base_hash = unsafe { base.0.__bindgen_anon_1.name.hash };
        let base_length = unsafe { base.0.__bindgen_anon_1.name.length };
        assert!((base_length as usize + str.len()) <= u8::MAX as usize);
        Self(red::TweakDBID {
            __bindgen_anon_1: red::TweakDBID__bindgen_ty_1 {
                name: red::TweakDBID__bindgen_ty_1__bindgen_ty_1 {
                    hash: crc32_seed(str.as_bytes(), base_hash),
                    length: str.len() as u8 + base_length,
                    tdbOffsetBE: [0, 0, 0],
                },
            },
        })
    }
}

impl Hash for TweakDbId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe { self.0.__bindgen_anon_1.value }.hash(state);
    }
}

impl TweakDbId {
    pub fn is_valid(self) -> bool {
        unsafe { self.0.IsValid() }
    }

    pub fn has_tdb_offset(self) -> bool {
        self.to_tdb_offset() != 0
    }

    pub fn to_tdb_offset(self) -> i32 {
        BigEndian::read_i24(unsafe { &self.0.__bindgen_anon_1.name.tdbOffsetBE }) as i32
    }

    pub fn set_tdb_offset(&mut self, offset: i32) {
        assert!(offset <= (i8::MAX as i32 * i8::MAX as i32 * i8::MAX as i32));
        assert!(offset >= (i8::MIN as i32 * i8::MIN as i32 * i8::MIN as i32));
        BigEndian::write_i24(
            unsafe { &mut self.0.__bindgen_anon_1.name.tdbOffsetBE },
            offset,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::TweakDbId;

    #[test]
    fn conversion() {
        assert_eq!(
            TweakDbId::new("Items.FirstAidWhiffV0"),
            TweakDbId::from(90_628_141_458)
        );
        assert_eq!(
            u64::from(TweakDbId::new("Items.FirstAidWhiffV0")),
            90_628_141_458
        );
    }

    #[test]
    fn mutation() {
        let mut original = TweakDbId::from(90_628_141_458);
        original.set_tdb_offset(128);
        assert_eq!(original.to_tdb_offset(), 128);
    }
}
