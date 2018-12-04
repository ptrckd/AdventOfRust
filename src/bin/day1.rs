use std::collections::HashSet;
use std::error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), Box<error::Error>> {
    let file = File::open("day1_input.txt")?;
    let input = BufReader::new(file);

    let mut frequencies = HashSet::new();
    let mut current_frequency = 0;
    frequencies.insert(current_frequency);

    let lines = input.lines().map(|line| line.unwrap());
    let frequency_changes: Vec<i32> = lines.map(|num| num.parse().unwrap()).collect();

    let resulting_freq: i32 = frequency_changes.iter().sum();
    println!("Resulting frequency: {}", resulting_freq);

    for frequency_change in frequency_changes.iter().cycle() {
        current_frequency += frequency_change;
        let seen_before = !frequencies.insert(current_frequency);

        if seen_before {
            println!("Repeated frequency: {}", current_frequency);
            break;
        }
    }

    Ok(())
}
