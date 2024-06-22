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
        if let Some(hour) = self.hour().checked_add(hour) {
            let mut copy = *self;
            unsafe { copy.0.AddHours(hour) };
            return Some(copy);
        }
        None
    }

    fn with_minute(&self, min: u32) -> Option<Self> {
        if let Some(min) = self.minute().checked_add(min) {
            let mut copy = *self;
            unsafe { copy.0.AddMinutes(min) };
            return Some(copy);
        }
        None
    }

    fn with_second(&self, sec: u32) -> Option<Self> {
        if let Some(sec) = self.second().checked_add(sec) {
            let mut copy = *self;
            unsafe { copy.0.AddSeconds(sec) };
            return Some(copy);
        }
        None
    }

    /// GameTime does not support nanoseconds, so it will always return `None`
    fn with_nanosecond(&self, _: u32) -> Option<Self> {
        None
    }

    fn hour12(&self) -> (bool, u32) {
        let hour = self.hour();
        let mut hour12 = hour % 12;
        if hour12 == 0 {
            hour12 = 12;
        }
        (hour >= 12, hour12)
    }

    fn num_seconds_from_midnight(&self) -> u32 {
        self.hour() * 3600 + self.minute() * 60 + self.second()
    }
}
