extern crate chrono;
extern crate regex;

use chrono::prelude::*;
use regex::Regex;

use std::cmp::Ordering;
use std::error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct GuardEvent {
    date_time: DateTime<Utc>,
    action: GuardAction,
}

impl PartialOrd for GuardEvent {
    fn partial_cmp(&self, other: &GuardEvent) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GuardEvent {
    fn cmp(&self, other: &GuardEvent) -> Ordering {
        self.date_time.cmp(&other.date_time)
    }
}

impl PartialEq for GuardEvent {
    fn eq(&self, other: &GuardEvent) -> bool {
        self.date_time == other.date_time
    }
}
impl Eq for GuardEvent {}

#[derive(Debug)]
enum GuardAction {
    StartShift(i32),
    FallAsleep,
    WakeUp,
}

fn parse_guard_action(action: &str) -> GuardAction {
    if action.starts_with("fa") {
        GuardAction::FallAsleep
    } else if action.starts_with("wa") {
        GuardAction::WakeUp
    } else {
        let x: i32 = action
            .chars()
            .filter(|c| is_digit(c))
            .collect::<String>()
            .parse()
            .unwrap();

        GuardAction::StartShift(x)
    }
}

fn is_digit(c: &char) -> bool {
    match c {
        '0'...'9' => true,
        _ => false,
    }
}

fn main() -> Result<(), Box<error::Error>> {
    let file = File::open("day4_input.txt")?;
    let input = BufReader::new(file);

    let re = Regex::new(r"\[(\d{4}\-\d{2}\-\d{2}\s\d{2}:\d{2})\]\s(.*)").unwrap();

    let lines: Result<Vec<String>, std::io::Error> = input.lines().collect();
    let lines = lines.expect("Issue reading lines from file");

    let mut events: Vec<GuardEvent> = lines
        .iter()
        .map(|line| re.captures(line).unwrap())
        .map(|capture| {
            let date_time_str = capture.get(1).unwrap().as_str();
            let date_time = Utc.datetime_from_str(date_time_str, "%F %R").unwrap();
            let action_str = capture.get(2).unwrap().as_str();
            let action = parse_guard_action(action_str);

            GuardEvent { date_time, action }
        }).collect();

    events.sort();

    println!("{:?}", events[0]);
    Ok(())
}
