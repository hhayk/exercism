use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");

        let total_miuntes_in_day = 24 * 60;
        let total_minutes = hours * 60 + minutes;

        let normalized_minutes =
            (total_minutes % total_miuntes_in_day + total_miuntes_in_day) % total_miuntes_in_day;

        Self {
            hours: normalized_minutes / 60,
            minutes: normalized_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");
        let total_minutes = self.hours * 60 + self.minutes;
        let new_total_minutes = total_minutes + minutes;

        Self::new(0, new_total_minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
