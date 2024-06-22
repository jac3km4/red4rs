use chrono::Timelike;

use crate::raw::root::RED4ext as red;

#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct GameTime(red::GameTime);

#[cfg(not(test))]
impl std::fmt::Display for GameTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [day, hour, min, sec] = unsafe { self.0.ToString() };
        write!(f, "{day}T{hour}:{min}:{sec}")
    }
}

#[cfg(feature = "chrono")]
impl From<GameTime> for chrono::DateTime<chrono::Utc> {
    fn from(value: GameTime) -> Self {
        // SAFETY: seconds being u32 it fits in i64, and nanos are zero
        Self::from_timestamp(value.0.seconds as i64, 0).unwrap()
    }
}

#[cfg(feature = "chrono")]
impl From<chrono::DateTime<chrono::Utc>> for GameTime {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self(red::GameTime {
            seconds: value.second(),
        })
    }
}
