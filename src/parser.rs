use chrono::*;

trait DateTimeParser {
    fn parse_datetime(&self, text: &str) -> Result<DateTime<FixedOffset>, ParseError>;
}

struct ChronoDateTimeParser {
    line_format: String
}

impl ChronoDateTimeParser {
    fn new(line_format: &str) -> Option<ChronoDateTimeParser> {
        if !line_format.contains("%t") {
            return None;
        }
        if !line_format.contains("%d") {
            return None;
        }
        Some(ChronoDateTimeParser{ line_format: line_format.to_string() })
    }
}

impl DateTimeParser for ChronoDateTimeParser {
    fn parse_datetime(&self, text: &str) -> Result<DateTime<FixedOffset>, ParseError> {
        let first_formatter = &self.line_format[1..2];
        let pure_date_time_text = str::replace(text, " some static text", "");
        if first_formatter == "t" {
            DateTime::parse_from_str(text, "%H:%M:%S %z:%d/%b/%Y")
        }
        else {
            DateTime::parse_from_str(pure_date_time_text.as_str(), "%d/%b/%Y:%H:%M:%S %z")
        }
    }
}

#[test]
fn can_handle_time_and_date_being_the_only_text() {
    let line = "10/Sep/2016:18:35:47 +0200";
    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line);
    assert!(timestamp.is_ok());
}

#[test]
fn can_handle_static_text_before_the_formatters() {
    let line = "some static text before 10/Sep/2016:18:35:47 +0200";
    let parser = ChronoDateTimeParser::new("some static text before %d:%t").unwrap();
    let timestamp = parser.parse_datetime(line);
    assert!(timestamp.is_ok());
}

#[test]
fn can_handle_static_text_after_the_formatters() {
    let line = "10/Sep/2016:18:35:47 +0200 some static text";
    let parser = ChronoDateTimeParser::new("%d:%t some static text").unwrap();
    let timestamp = parser.parse_datetime(line);
    assert!(timestamp.is_ok());
}

#[test]
fn has_proper_time_when_time_before_date() {
    let parser = ChronoDateTimeParser::new("%t:%d").unwrap();
    let line = "18:35:47 +0200:10/Sep/2016";
    let timestamp = parser.parse_datetime(line).unwrap();
    let time = timestamp.time();
    assert_eq!(time, NaiveTime::from_hms(18, 35, 47))
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
    let line = "10/Sep/2016:18:35:47 +0200";
    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line).unwrap();
    let timezone = timestamp.timezone();
    assert_eq!(timezone, FixedOffset::east(2 * 3600))
}

#[test]
fn has_proper_negative_timezone() {
    let line = "10/Sep/2016:18:35:47 -0200";
    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line).unwrap();
    let timezone = timestamp.timezone();
    assert_eq!(timezone, FixedOffset::west(2 * 3600))
}

#[test]
fn has_proper_date_when_midnight() {
    let line = "10/Sep/2016:00:00:00 +0200";
    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line).unwrap();
    let date = timestamp.date();
    assert_eq!(date, UTC.ymd(2016, 09, 10))
}

#[test]
fn has_proper_date() {
    let line = "10/Sep/2016:18:35:47 +0200";

    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line).unwrap();
    let date = timestamp.date();
    assert_eq!(date, UTC.ymd(2016, 09, 10))
}

#[test]
fn has_proper_time() {
    let line = "10/Sep/2016:18:35:47 +0200";
    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line).unwrap();
    let time = timestamp.time();
    assert_eq!(time, NaiveTime::from_hms(18, 35, 47))
}

#[test]
fn has_proper_time_when_midnight() {
    let line = "10/Sep/2016:00:00:00 +0200";
    let parser = ChronoDateTimeParser::new("%d:%t").unwrap();
    let timestamp = parser.parse_datetime(line).unwrap();
    let time = timestamp.time();
    assert_eq!(time, NaiveTime::from_hms(0, 0, 0))
}
