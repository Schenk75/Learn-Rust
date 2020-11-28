use std::string::ToString;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hour, minute) = Self::format_time(hours, minutes);

        Clock {hour, minute}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hour, minute) = Self::format_time(self.hour, self.minute + minutes);

        Clock {hour, minute}
    }

    pub fn format_time(hours: i32, minutes: i32) -> (i32, i32) {
        let mut hour = hours;
        let mut minute = minutes;
    
        while minute < 0 {
            minute += 60;
            hour -= 1;
        }
        while minute >=60 {
            minute -= 60;
            hour += 1;
        }
    
        while hour < 0 {
            hour += 24;
        }
        hour %= 24;
    
        (hour, minute)
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }
}

