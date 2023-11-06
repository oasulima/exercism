use time::{format_description, Duration, Time};

#[derive(Debug, PartialEq)]
pub struct Clock {
    time: Time,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut time = Time::MIDNIGHT;
        time += Duration::hours(hours.into());
        time += Duration::minutes(minutes.into());

        Self { time }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            time: self.time + Duration::minutes(minutes.into()),
        }
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        let format = format_description::parse("[hour]:[minute]").unwrap();
        self.time.format(&format).unwrap()
    }
}
