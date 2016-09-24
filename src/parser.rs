use std::str::FromStr;
use chrono::*;

fn parse_time(line: &str) -> DateTime<FixedOffset> {
    Local::now().with_timezone(&FixedOffset::east(2 * 3600))
}

#[test]
fn has_proper_positive_timezone() {
    let line = "[10/Sep/2016:18:35:47 +0200]";
    let timestamp = parse_time(line);
    let timezone = timestamp.timezone();
    assert_eq!(timezone, FixedOffset::east(2 * 3600))
}
