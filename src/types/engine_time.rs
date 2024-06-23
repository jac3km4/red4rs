use crate::raw::root::RED4ext as red;

#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct EngineTime(red::EngineTime);

impl EngineTime {
    pub fn is_valid(&self) -> bool {
        self.0.unk00.ne(&[0; 8])
    }
}

impl PartialEq for EngineTime {
    fn eq(&self, other: &Self) -> bool {
        self.0.unk00.eq(&other.0.unk00)
    }
}

impl PartialOrd for EngineTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.unk00.partial_cmp(&other.0.unk00)
    }
}

impl From<f64> for EngineTime {
    fn from(value: f64) -> Self {
        Self(red::EngineTime {
            unk00: value.to_ne_bytes(),
        })
    }
}

impl From<EngineTime> for f64 {
    fn from(EngineTime(red::EngineTime { unk00 }): EngineTime) -> Self {
        Self::from_ne_bytes(unk00)
    }
}
