use std::hash::Hash;

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

impl PartialEq for GameTime {
    fn eq(&self, other: &Self) -> bool {
        self.0.seconds.eq(&other.0.seconds)
    }
}

impl Eq for GameTime {}

impl PartialOrd for GameTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GameTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.seconds.cmp(&other.0.seconds)
    }
}

impl Hash for GameTime {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.seconds.hash(state);
    }
}

impl From<u32> for GameTime {
    fn from(seconds: u32) -> Self {
        Self(red::GameTime { seconds })
    }
}

impl From<GameTime> for u32 {
    fn from(value: GameTime) -> Self {
        value.0.seconds
    }
}

#[cfg(feature = "chrono")]
impl From<GameTime> for chrono::DateTime<chrono::Utc> {
    fn from(value: GameTime) -> Self {
        // SAFETY: seconds being u32 it fits in i64, and nanos are zero
        Self::from_timestamp(value.0.seconds as i64, 0).unwrap()
    }
}

#[cfg(all(feature = "chrono", not(test)))]
impl From<chrono::DateTime<chrono::Utc>> for GameTime {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        use chrono::Timelike;
        Self(red::GameTime {
            seconds: value.second(),
        })
    }
}

#[cfg(all(feature = "chrono", not(test)))]
impl chrono::Timelike for GameTime {
    fn hour(&self) -> u32 {
        unsafe { self.0.GetHour() }
    }

    fn minute(&self) -> u32 {
        unsafe { self.0.GetMinute() }
    }

    fn second(&self) -> u32 {
        unsafe { self.0.GetSecond() }
    }

    fn nanosecond(&self) -> u32 {
        0
    }

    fn with_hour(&self, hour: u32) -> Option<Self> {
        if (0..=23).contains(&hour) {
            let mut copy = *self;
            unsafe { copy.0.SetHour(hour) };
            return Some(copy);
        }
        None
    }

    fn with_minute(&self, min: u32) -> Option<Self> {
        if (0..=59).contains(&min) {
            let mut copy = *self;
            unsafe { copy.0.SetMinute(min) };
            return Some(copy);
        }
        None
    }

    fn with_second(&self, sec: u32) -> Option<Self> {
        if (0..=59).contains(&sec) {
            let mut copy = *self;
            unsafe { copy.0.SetSecond(sec) };
            return Some(copy);
        }
        None
    }

    /// GameTime does not support nanoseconds, so it will always return `None`
    fn with_nanosecond(&self, _: u32) -> Option<Self> {
        None
    }
}
