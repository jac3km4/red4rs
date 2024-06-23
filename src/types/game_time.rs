use std::hash::Hash;

use crate::raw::root::RED4ext as red;

#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct GameTime(red::GameTime);

impl GameTime {
    pub fn new(days: u32, hours: u32, minutes: u32, seconds: u32) -> Self {
        let mut this = Self::default();
        this.add_days(days);
        this.add_hours(hours);
        this.add_minutes(minutes);
        this.add_seconds(seconds);
        this
    }

    pub fn add_days(&mut self, days: u32) {
        self.0.seconds = self.0.seconds.saturating_add(
            days.saturating_mul(24)
                .saturating_mul(60)
                .saturating_mul(60),
        );
    }

    pub fn add_hours(&mut self, hours: u32) {
        self.0.seconds = self
            .0
            .seconds
            .saturating_add(hours.saturating_mul(60).saturating_mul(60));
    }

    pub fn add_minutes(&mut self, minutes: u32) {
        self.0.seconds = self.0.seconds.saturating_add(minutes.saturating_mul(60));
    }

    pub fn add_seconds(&mut self, seconds: u32) {
        self.0.seconds = self.0.seconds.saturating_add(seconds);
    }

    pub fn day(&self) -> u32 {
        unsafe { self.0.GetDay() }
    }

    pub fn hour(&self) -> u32 {
        unsafe { self.0.GetHour() }
    }

    pub fn minute(&self) -> u32 {
        unsafe { self.0.GetMinute() }
    }

    pub fn second(&self) -> u32 {
        unsafe { self.0.GetSecond() }
    }
}

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

#[cfg(feature = "chrono")]
impl From<chrono::DateTime<chrono::Utc>> for GameTime {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        use chrono::Timelike;
        Self(red::GameTime {
            seconds: value.second(),
        })
    }
}

#[cfg(feature = "chrono")]
impl chrono::Timelike for GameTime {
    fn hour(&self) -> u32 {
        Self::hour(self)
    }

    fn minute(&self) -> u32 {
        Self::minute(self)
    }

    fn second(&self) -> u32 {
        Self::second(self)
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

#[cfg(test)]
mod tests {
    use super::GameTime;

    #[test]
    fn instantiation() {
        let time = GameTime::new(2, 0, 7, 7);
        assert_eq!(time.day(), 2);
        assert_eq!(time.hour(), 0);
        assert_eq!(time.minute(), 7);
        assert_eq!(time.second(), 7);

        #[cfg(feature = "chrono")]
        {
            use chrono::Timelike;
            let time = time.with_minute(2).unwrap().with_second(2).unwrap();
            assert_eq!(time.day(), 2);
            assert_eq!(time.hour(), 0);
            assert_eq!(time.minute(), 2);
            assert_eq!(time.second(), 2);
        }
    }
}
