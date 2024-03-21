use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub struct Date {
    duration: Duration
}

impl Date {
    pub fn new() -> Date {
        let system_time = SystemTime::now();
        let duration = system_time.duration_since(UNIX_EPOCH).expect("duration could not be parsed");
        Date{duration}
    }
    pub fn year(&self) -> u64 {
        let seconds_in_a_year = 60*60*24*365;
        1970 + self.duration.as_secs()/seconds_in_a_year
    }
}

