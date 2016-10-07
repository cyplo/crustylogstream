use chrono::*;

trait DateTimeParser {
    fn parse_datetime(&mut self, text: &str) -> Result<DateTime<FixedOffset>, ParseError>;
}

struct ChronoDateTimeParser {
}

impl ChronoDateTimeParser {
    fn new() -> ChronoDateTimeParser {
        ChronoDateTimeParser{}
    }
}

impl DateTimeParser for ChronoDateTimeParser {
    fn parse_datetime(&mut self, text: &str) -> Result<DateTime<FixedOffset>, ParseError> {
        DateTime::parse_from_str(text, "[%d/%b/%Y:%H:%M:%S %z]")
    }
}

struct FakeDateTimeParser {
    used: bool
}

impl DateTimeParser for FakeDateTimeParser {
    fn parse_datetime(&mut self, text: &str) -> Result<DateTime<FixedOffset>, ParseError> {
        self.used = true;
        Ok(DateTime::from_utc(Local::now().naive_local(), FixedOffset::east(0)))
    }
}

impl FakeDateTimeParser {
    fn new() -> FakeDateTimeParser {
        FakeDateTimeParser { used: false }
    }

    fn is_used(&self) -> bool {
        self.used
    }
}

fn parse_datetime(text: &str) -> Result<DateTime<FixedOffset>, ParseError> {
    let mut parser = ChronoDateTimeParser::new();
    parser.parse_datetime(text)
}

fn parse(text: &str, format: &str, parser: &mut FakeDateTimeParser) -> Result<(), ()> {
    parser.parse_datetime("");
    Ok(())
}

#[test]
fn returns_result() {
    let line = "[18:35:47 +0200]";
    let format = "[%t]";
    let mut datetime_parser = FakeDateTimeParser::new();
    let parsed = parse(line, format, &mut datetime_parser);

    assert!(parsed.is_ok());
}

#[test]
fn uses_datetime_parser() {
    let line = "[18:35:47 +0200]";
    let format = "[%t]";
    let mut datetime_parser = FakeDateTimeParser::new();
    let parsed = parse(line, format, &mut datetime_parser);

    assert!(datetime_parser.is_used());
}

#[test]
fn has_proper_positive_timezone() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let timestamp = parse_datetime(line).unwrap();
    let timezone = timestamp.timezone();
    assert_eq!(timezone, FixedOffset::east(2 * 3600))
}

#[test]
fn has_proper_negative_timezone() {
    let line = "[10/Sep/2016:18:35:47 -0200]";
    let timestamp = parse_datetime(line).unwrap();
    let timezone = timestamp.timezone();
    assert_eq!(timezone, FixedOffset::west(2 * 3600))
}

#[test]
fn has_proper_date_when_midnight() {
    let line = "[10/Sep/2016:00:00:00 +0200]";
    let timestamp = parse_datetime(line).unwrap();
    let date = timestamp.date();
    assert_eq!(date, UTC.ymd(2016, 09, 10))
}

#[test]
fn has_proper_date() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let timestamp = parse_datetime(line).unwrap();
    let date = timestamp.date();
    assert_eq!(date, UTC.ymd(2016, 09, 10))
}

#[test]
fn has_proper_time() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let timestamp = parse_datetime(line).unwrap();
    let time = timestamp.time();
    assert_eq!(time, NaiveTime::from_hms(18, 35, 47))
}

#[test]
fn has_proper_time_when_midnight() {
    let line = "[10/Sep/2016:00:00:00 +0200]";
    let timestamp = parse_datetime(line).unwrap();
    let time = timestamp.time();
    assert_eq!(time, NaiveTime::from_hms(0, 0, 0))
}
