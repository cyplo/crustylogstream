use std::str::FromStr;
use chrono::*;

fn parse_time(line: &str) -> Result<DateTime<FixedOffset>, ParseError> {
    let date_format = "%d/%b/%Y";
    let time_format = "%H:%M:%S";
    let timezone_format = "%z";
    let timestamp_format = date_format + ":" + time_format + " " + timezone_format;
    let mut parsed_data = format::  Parsed::new();
    let parse_result = parse(&parsed_data, line);
    DateTime::from_str(line)
}

#[test]
fn timestamp_should_have_proper_time_offset() {
    let line = "10/Sep/2016T18:35:47 +0200";
    let timestamp = parse_time(line).unwrap();
    let timezone = timestamp.timezone();
    assert_eq!(timezone, FixedOffset::east(2 * 3600))
}
