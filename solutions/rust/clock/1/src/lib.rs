use std::fmt;
#[derive(Debug, PartialEq, Eq)]
pub struct Clock{
    minutes : i32,
}

impl Clock {
    const MINUTES_IN_A_DAY :i32 = 24 * 60;
    
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        Clock{
            minutes: total_minutes.rem_euclid(Self::MINUTES_IN_A_DAY),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.minutes / 60;
        let mins = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, mins)
    }
}