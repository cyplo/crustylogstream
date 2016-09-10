use std::str::FromStr;
use chrono::*;

fn parse_time(line: &str) -> DateTime<FixedOffset> {
    DateTime::from_str(line).unwrap()
}

#[test]
fn can_parse_timestamp() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let timestamp = parse_time(line);
    let timezone = timestamp.timezone();
    assert_eq!(timezone, FixedOffset::east(2 * 3600))
}
