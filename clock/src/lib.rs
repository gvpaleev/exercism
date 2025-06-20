use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes: i32 = hours * 60 + minutes;
        let normalized_minutes: i32 = total_minutes.rem_euclid(1440);
        Clock { minutes: normalized_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes: i32 = self.minutes + minutes;
        let normalized_minutes: i32 = total_minutes.rem_euclid(1440);
        Clock { minutes: normalized_minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours: i32 = self.minutes / 60;
        let minutes: i32 = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
