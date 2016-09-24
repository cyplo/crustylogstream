use std::str::FromStr;
use chrono::*;

fn parse_time(text: &str) -> Result<DateTime<FixedOffset>, ParseError> {
    DateTime::parse_from_str(text, "[%d/%b/%Y:%H:%M:%S %z]")
}

#[test]
fn has_proper_positive_timezone() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let timestamp = parse_time(line).unwrap();
    let timezone = timestamp.timezone();
    assert_eq!(timezone, FixedOffset::east(2 * 3600))
}

#[test]
fn has_proper_negative_timezone() {
    let line = "[10/Sep/2016:18:35:47 -0200]";
    let timestamp = parse_time(line).unwrap();
    let timezone = timestamp.timezone();
    assert_eq!(timezone, FixedOffset::west(2 * 3600))
}
#[test]
fn has_proper_date_when_midnight() {
    let line = "[10/Sep/2016:00:00:00 +0200]";
    let timestamp = parse_time(line).unwrap();
    let date = timestamp.date();
    assert_eq!(date, UTC.ymd(2016,09,10))
}

#[test]
fn has_proper_date() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let timestamp = parse_time(line).unwrap();
    let date = timestamp.date();
    assert_eq!(date, UTC.ymd(2016,09,10))
}

#[test]
fn has_proper_time() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let timestamp = parse_time(line).unwrap();
    let time = timestamp.time();
    assert_eq!(time, NaiveTime::from_hms(18,35,47))
}

#[test]
fn has_proper_time_when_midnight() {
    let line = "[10/Sep/2016:00:00:00 +0200]";
    let timestamp = parse_time(line).unwrap();
    let time = timestamp.time();
    assert_eq!(time, NaiveTime::from_hms(0,0,0))
}
