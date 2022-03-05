use chrono::prelude::*;
use crate::number::number::some_number_between;

pub fn some_date_time() -> DateTime<Local> {
    Local.timestamp_millis(some_number_between(1, 32503683600000))
}

pub fn some_utc_date_time() -> DateTime<Utc> {
    Utc.timestamp_millis(some_number_between(1, 32503683600000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_some_local_date_time() {
        let date_time: DateTime<Local> = some_date_time();
        assert_ne!(date_time, Local.timestamp_millis(0));
    }

    #[test]
    fn can_generate_some_local_date_time_before() {
        let date_time: DateTime<Local> = some_date_time();
        assert_ne!(date_time, Local.timestamp_millis(0));
    }

    #[test]
    fn can_generate_some_local_date_time_after() {
        let date_time: DateTime<Local> = some_date_time();
        assert_ne!(date_time, Local.timestamp_millis(0));
    }

    #[test]
    fn can_generate_some_utc_local_date_time() {
        let date_time: DateTime<Utc> = some_utc_date_time();
        assert_ne!(date_time, Utc.timestamp_millis(0));
    }
}