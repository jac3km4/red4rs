use std::hash::Hash;

use crate::fnv1a32;
use crate::raw::root::RED4ext as red;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Cruid(red::CRUID);

impl Cruid {
    #[inline]
    pub const fn is_defined(self) -> bool {
        self.0.unk00 != 0
    }

    pub const fn new(str: &str) -> Self {
        Self(red::CRUID {
            unk00: 0xF000000000000000 as u64 as i64 | (fnv1a32(str) << 2) as i64,
        })
    }
}

impl PartialEq for Cruid {
    fn eq(&self, other: &Self) -> bool {
        self.0.unk00 == other.0.unk00
    }
}

impl Eq for Cruid {}

impl PartialOrd for Cruid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cruid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.unk00.cmp(&other.0.unk00)
    }
}

impl Hash for Cruid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.unk00.hash(state);
    }
}

impl Default for Cruid {
    fn default() -> Self {
        Self(red::CRUID { unk00: 0 })
    }
}

impl From<i64> for Cruid {
    fn from(hash: i64) -> Self {
        Self(red::CRUID { unk00: hash })
    }
}

impl From<Cruid> for i64 {
    fn from(Cruid(red::CRUID { unk00 }): Cruid) -> Self {
        unk00
    }
}
