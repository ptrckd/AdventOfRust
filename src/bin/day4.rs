extern crate chrono;
extern crate regex;

use chrono::prelude::*;
use chrono::Duration;

use regex::Regex;

use std::cmp::Ordering;
use std::collections::HashMap;
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

    let guard_sleep_times = guard_sleep_time(&events);

    let sleepy_guard = get_sleepiest_guard(&guard_sleep_times);

    let minutes_asleep = guard_sleep_times.get(&sleepy_guard).unwrap();

    let mut sleepiest_minute = 0;
    let mut most_times = 0;

    for (i, times) in minutes_asleep.iter().enumerate() {
        if *times > most_times {
            sleepiest_minute = i;
            most_times = *times;
        }
    }

    println!(
        "Guard: {}, slept the most and is most likely to be asleep during minute {}",
        sleepy_guard, sleepiest_minute
    );

    println!("The code is {}", sleepy_guard * sleepiest_minute as i32);

    let mut most_slept_minute = 0;
    let mut number_of_times = 0;
    let mut guard = 0;

    for (guard_id, times) in guard_sleep_times.iter() {
        for (i, times) in times.iter().enumerate() {
            if *times > number_of_times {
                guard = *guard_id;
                most_slept_minute = i;
                number_of_times = *times;
            }
        }
    }

    println!(
        "Guard {} is most frequently asleep on minute {}. It happened {} times.",
        guard, most_slept_minute, number_of_times
    );
    println!("The code is {}", guard * most_slept_minute as i32);

    Ok(())
}

fn guard_sleep_time(events: &Vec<GuardEvent>) -> HashMap<i32, [u32; 60]> {
    let mut time_asleep: HashMap<i32, [u32; 60]> = HashMap::new();

    let mut guard_id = 0;
    let mut previous_event = events.get(0).unwrap();

    for event in events {
        match event.action {
            GuardAction::StartShift(id) => {
                if let GuardAction::FallAsleep = previous_event.action {
                    let mut duration = time_asleep.entry(guard_id).or_insert([0; 60]);
                    for minute in previous_event.date_time.minute()..event.date_time.minute() {
                        duration[minute as usize] += 1;
                    }
                }

                guard_id = id;
                previous_event = event;
            }
            GuardAction::FallAsleep => {
                previous_event = event;
            }
            GuardAction::WakeUp => {
                if let GuardAction::FallAsleep = previous_event.action {
                    let mut duration = time_asleep.entry(guard_id).or_insert([0; 60]);
                    for minute in previous_event.date_time.minute()..event.date_time.minute() {
                        duration[minute as usize] += 1;
                    }
                }
                previous_event = event;
            }
        }
    }

    time_asleep
}

fn get_sleepiest_guard(sleep_times: &HashMap<i32, [u32; 60]>) -> i32 {
    let mut max_sleep_time = 0;
    let mut guard = 0;

    for (key, value) in sleep_times.iter() {
        let sleep_time = value.iter().sum();
        if sleep_time > max_sleep_time {
            max_sleep_time = sleep_time;
            guard = *key;
        }
    }

    guard
}
