use std::error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), Box<error::Error>> {
    let file = File::open("day2_input.txt")?;
    let input = BufReader::new(file);

    let lines: Vec<String> = input.lines().map(|line| line.unwrap()).collect();

    let mut two_repeat = 0;
    let mut three_repeat = 0;

    for line in &lines {
        let mut has_two = 0;
        let mut has_three = 0;
        let chars = line.chars();
        for character in chars {
            let matches = line.matches(character).collect::<Vec<_>>().len();
            if matches == 2 {
                has_two = 1;
            }
            if matches == 3 {
                has_three = 1;
            }
        }
        two_repeat += has_two;
        three_repeat += has_three;
    }
    println!("Boxes with two repeating elements: {}", two_repeat);
    println!("Boxes with three repeating elements: {}", three_repeat);
    println!(
        "Checksum: {} x {} = {}",
        two_repeat,
        three_repeat,
        two_repeat * three_repeat
    );

    print!("The common letters are: ");
    for line in &lines {
        let similar_line: Vec<String> = lines
            .iter()
            .filter(|line_b| one_char_different(line, line_b, false))
            .map(|line| line.to_string())
            .collect();

        if similar_line.len() > 0 {
            let ocd = &similar_line[0];
            one_char_different(line, &ocd, true);
            print!("\n");
            break;
        }
    }

    Ok(())
}

fn one_char_different(string_x: &str, string_y: &str, print: bool) -> bool {
    let mut once = false;
    for (x, y) in string_x.chars().zip(string_y.chars()) {
        if x != y {
            if once {
                return false;
            }
            once = true;
        } else {
            if print {
                print!("{}", x);
            }
        }
    }
    once
}
