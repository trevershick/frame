extern crate regex;
use chrono::Utc;
use regex::Regex;
use std::str;
use std::time::Duration;

pub type Frame = [i64; 3];

pub trait ToDuration {
    fn to_duration(&self) -> Result<std::time::Duration, String>;
}
pub trait ToFrame {
    fn to_frame(&self) -> Result<Frame, String>;
}
impl ToDuration for &str {
    fn to_duration(&self) -> Result<std::time::Duration, String> {
        parse_duration(self)
    }
}

impl ToDuration for String {
    fn to_duration(&self) -> Result<std::time::Duration, String> {
        parse_duration(self.as_str())
    }
}

impl ToFrame for &str {
    fn to_frame(&self) -> Result<Frame, String> {
        parse_frame(self)
    }
}

impl ToFrame for String {
    fn to_frame(&self) -> Result<Frame, String> {
        parse_frame(self.as_str())
    }
}

fn multiplier(text: &str) -> u64 {
    match text {
        "s" => 1,
        "m" => 60 * multiplier("s"),
        "h" => 60 * multiplier("m"),
        "d" => 24 * multiplier("h"),
        "w" => 7 * multiplier("d"),
        "y" => 365 * multiplier("d"),
        _ => panic!(format!("Invalid unit {}", text)),
    }
}

pub fn parse_duration(text: &str) -> Result<std::time::Duration, String> {
    let re = Regex::new(r"^(\d+)([smhwdy])$").unwrap();
    match re.captures(text) {
        Some(caps) => {
            let value = caps.get(1).unwrap().as_str();
            let value = value.parse::<u64>().unwrap();
            let unit = caps.get(2).unwrap().as_str();
            Result::Ok(Duration::from_secs(value * multiplier(unit)))
        }
        None => Err(String::from("invalid specification")),
    }
}

pub fn parse_frame(text: &str) -> Result<Frame, String> {
    let n = Utc::now();
    let s = text.to_duration()?.as_secs() as i64;
    Ok([n.timestamp() - s, n.timestamp(), s])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_s() {
        assert_eq!(parse_duration("2s").unwrap().as_secs(), 2);
    }
    #[test]
    fn unit_m() {
        assert_eq!(parse_duration("1m").unwrap().as_secs(), 60);
        assert_eq!(parse_duration("27m").unwrap().as_secs(), 27 * 60);
    }
    #[test]
    fn unit_h() {
        assert_eq!(parse_duration("1h").unwrap().as_secs(), 60 * 60);
        assert_eq!(parse_duration("27h").unwrap().as_secs(), 27 * 60 * 60);
    }
    #[test]
    fn unit_y() {
        assert_eq!(
            parse_duration("2y").unwrap().as_secs(),
            2 * 365 * parse_duration("1d").unwrap().as_secs()
        );
    }
}
