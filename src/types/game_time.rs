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
