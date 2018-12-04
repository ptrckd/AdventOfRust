extern crate regex;
use regex::Regex;

use std::collections::HashSet;
use std::error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct ClaimedArea {
    id: u32,
    north_edge: u32,
    east_edge: u32,
    width: u32,
    height: u32,
    claimed_points: HashSet<(u32, u32)>,
}

impl ClaimedArea {
    //east, north, width, height
    fn new(captures: &regex::Captures) -> Result<ClaimedArea, std::num::ParseIntError> {
        let id = captures.get(1).unwrap().as_str().parse()?;
        let north_edge = captures.get(3).unwrap().as_str().parse()?;
        let east_edge = captures.get(2).unwrap().as_str().parse()?;
        let width = captures.get(4).unwrap().as_str().parse()?;
        let height = captures.get(5).unwrap().as_str().parse()?;

        let mut claimed_points: HashSet<(u32, u32)> = HashSet::new();

        for x in east_edge..(east_edge + width) {
            for y in north_edge..(north_edge + height) {
                claimed_points.insert((x, y));
            }
        }

        Ok(ClaimedArea {
            id,
            north_edge,
            east_edge,
            width,
            height,
            claimed_points,
        })
    }

    fn overlapping_points<'a>(&'a self, other: &'a ClaimedArea) -> HashSet<&'a (u32, u32)> {
        self.claimed_points
            .intersection(&other.claimed_points)
            .collect()
    }
}

fn main() -> Result<(), Box<error::Error>> {
    let file = File::open("day3_input.txt")?;
    let input = BufReader::new(file);

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let lines: Result<Vec<String>, std::io::Error> = input.lines().collect();
    let lines = lines.expect("Issue reading lines from file");

    let matches: Vec<regex::Captures> = lines
        .iter()
        .map(|line| re.captures(line).unwrap())
        .collect();

    let claimed_areas: Vec<ClaimedArea> = matches
        .iter()
        .map(|r_match| ClaimedArea::new(r_match).unwrap())
        .collect();

    let mut all_overlapping_points: HashSet<&(u32, u32)> = HashSet::new();
    let mut read = 1;

    let len = claimed_areas.len();

    for x in claimed_areas.iter().take(len - 1) {
        for y in claimed_areas.iter().skip(read) {
            let overlapping_points = x.overlapping_points(y);
            for point in &overlapping_points {
                all_overlapping_points.insert(point);
            }
        }
        read += 1;
    }

    println!("Total overlapping points: {}", all_overlapping_points.len());

    for claimed_area in &claimed_areas {
        let mut overlap = false;
        for point in &claimed_area.claimed_points {
            if all_overlapping_points.contains(&point) {
                overlap = true;
            }
        }
        if !overlap {
            println!("No overlap on area: {}", claimed_area.id);
        }
    }

    Ok(())
}

// #15 @ 916,559: 29x27