use std::time::Duration;

use crate::raw::root::RED4ext as red;

#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct EngineTime(red::EngineTime);

impl EngineTime {
    pub fn is_valid(&self) -> bool {
        self.0.unk00 != [0; 8]
    }

    pub fn as_secs_f64(&self) -> f64 {
        f64::from_ne_bytes(self.0.unk00)
    }

    pub fn saturating_add_assign(&mut self, value: impl Into<f64>) {
        let current = self.as_secs_f64();
        let value: f64 = value.into();
        let addition = current + value;
        if addition.is_infinite() {
            if addition.is_sign_positive() {
                self.0.unk00 = f64::MAX.to_ne_bytes();
            } else {
                self.0.unk00 = f64::MIN.to_ne_bytes();
            }
        } else {
            self.0.unk00 = addition.to_ne_bytes();
        }
    }

    pub fn saturating_add(self, value: impl Into<f64>) -> Self {
        let mut copy = self;
        copy.saturating_add_assign(value.into());
        copy
    }

    pub fn saturating_sub_assign(&mut self, value: impl Into<f64>) {
        let current = self.as_secs_f64();
        let value: f64 = value.into();
        let addition = current - value;
        if addition.is_infinite() {
            if addition.is_sign_positive() {
                self.0.unk00 = f64::MAX.to_ne_bytes();
            } else {
                self.0.unk00 = f64::MIN.to_ne_bytes();
            }
        } else {
            self.0.unk00 = addition.to_ne_bytes();
        }
    }

    pub fn saturating_sub(self, value: impl Into<f64>) -> Self {
        let mut copy = self;
        copy.saturating_sub_assign(value.into());
        copy
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

impl TryFrom<f64> for EngineTime {
    type Error = EngineTimeError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_infinite() {
            return Err(EngineTimeError::OutOfBounds);
        }
        Ok(Self(red::EngineTime {
            unk00: value.to_ne_bytes(),
        }))
    }
}

impl From<EngineTime> for f64 {
    fn from(EngineTime(red::EngineTime { unk00 }): EngineTime) -> Self {
        Self::from_ne_bytes(unk00)
    }
}

impl TryFrom<Duration> for EngineTime {
    type Error = EngineTimeError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        let value = value.as_secs_f64();
        value.try_into()
    }
}

impl std::ops::Add<Duration> for EngineTime {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        self.saturating_add(rhs.as_secs_f64())
    }
}

impl std::ops::AddAssign<Duration> for EngineTime {
    fn add_assign(&mut self, rhs: Duration) {
        self.saturating_add_assign(rhs.as_secs_f64());
    }
}

impl std::ops::Sub<Duration> for EngineTime {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.saturating_sub(rhs.as_secs_f64())
    }
}

impl std::ops::SubAssign<Duration> for EngineTime {
    fn sub_assign(&mut self, rhs: Duration) {
        self.saturating_sub_assign(rhs.as_secs_f64());
    }
}

#[derive(Debug)]
pub enum EngineTimeError {
    OutOfBounds,
}

impl std::fmt::Display for EngineTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OutOfBounds => "unsupported infinite or negative infinite floating-point",
            }
        )
    }
}

impl std::error::Error for EngineTimeError {}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::EngineTime;

    #[test]
    fn bounds() {
        assert!(EngineTime::try_from(f64::INFINITY).is_err());
        assert!(EngineTime::try_from(f64::NEG_INFINITY).is_err());

        let before = EngineTime::try_from(f64::MAX).unwrap();
        let after = before + Duration::from_millis(1);
        assert_eq!(after.as_secs_f64(), f64::MAX);

        let before = EngineTime::try_from(f64::MIN).unwrap();
        let after = before - Duration::from_millis(1);
        assert_eq!(after.as_secs_f64(), f64::MIN);
    }

    #[test]
    fn math() {
        let mut time = EngineTime::try_from(3.2).unwrap();
        time += Duration::from_secs_f64(4.1);

        assert_eq!(time.as_secs_f64(), 7.3);
    }
}
