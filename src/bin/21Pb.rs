#[derive(Debug, PartialEq, Eq)]

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let tt: i32 = ((hours * 60) + minutes).rem_euclid(24 * 60);

        Clock {
            hours: tt / 60,
            minutes: tt % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
