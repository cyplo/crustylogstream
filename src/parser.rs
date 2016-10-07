use chrono::*;

trait DateTimeParser {
    fn parse_datetime(&self, text: &str) -> Result<DateTime<FixedOffset>, ParseError>;
}

struct ChronoDateTimeParser {
}

impl ChronoDateTimeParser {
    fn new(format: &str) -> Option<ChronoDateTimeParser> {
        if !format.contains("%t") {
            return None;
        }
        if !format.contains("%d") {
            return None;
        }
        Some(ChronoDateTimeParser{})
    }
}

impl DateTimeParser for ChronoDateTimeParser {
    fn parse_datetime(&self, text: &str) -> Result<DateTime<FixedOffset>, ParseError> {
        DateTime::parse_from_str(text, "[%d/%b/%Y:%H:%M:%S %z]")
    }
}

#[test]
fn does_not_fail_with_time_and_date_formats() {
    let parser = ChronoDateTimeParser::new("%t:%d");
    assert!(parser.is_some())
}

#[test]
fn fails_without_time_format() {
    let parser = ChronoDateTimeParser::new("%d");
    assert!(parser.is_none())
}

#[test]
fn fails_without_date_format() {
    let parser = ChronoDateTimeParser::new("%t");
    assert!(parser.is_none())
}

#[test]
fn has_proper_positive_timezone() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line).unwrap();
    let timezone = timestamp.timezone();
    assert_eq!(timezone, FixedOffset::east(2 * 3600))
}

#[test]
fn has_proper_negative_timezone() {
    let line = "[10/Sep/2016:18:35:47 -0200]";
    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line).unwrap();
    let timezone = timestamp.timezone();
    assert_eq!(timezone, FixedOffset::west(2 * 3600))
}

#[test]
fn has_proper_date_when_midnight() {
    let line = "[10/Sep/2016:00:00:00 +0200]";
    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line).unwrap();
    let date = timestamp.date();
    assert_eq!(date, UTC.ymd(2016, 09, 10))
}

#[test]
fn has_proper_date() {
    let line = "[10/Sep/2016:18:35:47 +0200]";

    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line).unwrap();
    let date = timestamp.date();
    assert_eq!(date, UTC.ymd(2016, 09, 10))
}

#[test]
fn has_proper_time() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line).unwrap();
    let time = timestamp.time();
    assert_eq!(time, NaiveTime::from_hms(18, 35, 47))
}

#[test]
fn has_proper_time_when_midnight() {
    let line = "[10/Sep/2016:00:00:00 +0200]";
    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line).unwrap();
    let time = timestamp.time();
    assert_eq!(time, NaiveTime::from_hms(0, 0, 0))
}
