use std::str::FromStr;
use chrono::*;

fn parse_time(line: &str) -> DateTime<FixedOffset> {
    UTC.ymd(2016,09,10).and_hms(16,35,47).with_timezone(&FixedOffset::east(2 * 3600))
}

#[test]
fn has_proper_positive_timezone() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let timestamp = parse_time(line);
    let timezone = timestamp.timezone();
    assert_eq!(timezone, FixedOffset::east(2 * 3600))
}

#[test]
fn has_proper_date() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let timestamp = parse_time(line);
    let date = timestamp.date();
    assert_eq!(date, UTC.ymd(2016,09,10))
}

#[test]
fn has_proper_time() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let timestamp = parse_time(line);
    let time = timestamp.time();
    assert_eq!(time, NaiveTime::from_hms(18,35,47))
}
