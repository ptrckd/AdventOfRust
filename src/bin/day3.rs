extern crate regex;
use regex::Regex;

use std::collections::HashSet;
use std::error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct ClaimedArea {
    id: u32,
    south_edge: u32,
    west_edge: u32,
    width: u32,
    height: u32,
    claimed_points: HashSet<(u32, u32)>,
}

impl ClaimedArea {
    //west, south, width, height
    fn new(captures: &regex::Captures) -> Result<ClaimedArea, std::num::ParseIntError> {
        let id = captures.get(1).unwrap().as_str().parse()?;
        let south_edge = captures.get(3).unwrap().as_str().parse()?;
        let west_edge = captures.get(2).unwrap().as_str().parse()?;
        let width = captures.get(4).unwrap().as_str().parse()?;
        let height = captures.get(5).unwrap().as_str().parse()?;

        let mut claimed_points: HashSet<(u32, u32)> = HashSet::new();

        for x in west_edge..(west_edge + width) {
            for y in south_edge..(south_edge + height) {
                claimed_points.insert((x, y));
            }
        }

        Ok(ClaimedArea {
            id,
            south_edge,
            west_edge,
            width,
            height,
            claimed_points,
        })
    }

    fn overlaps(&self, other: &ClaimedArea) -> bool {
        !((self.west_edge as i32) > (other.west_edge as i32 + other.width as i32)
            || (self.west_edge as i32 + self.width as i32) < (other.west_edge as i32)
            || (self.south_edge as i32 + self.height as i32) < (other.south_edge as i32)
            || (self.south_edge as i32) > (other.south_edge as i32 + other.height as i32))
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

    let lines: Result<Vec<String>, std::io::Error> = input.lines().collect();
    let lines = lines.expect("Issue reading lines from file");

    let matches = get_matches(&lines);

    let claimed_areas = get_areas(&matches);

    let all_overlapping_points = get_overlapping_points(&claimed_areas);

    println!("Total overlapping points: {}", all_overlapping_points.len());

    areas_no_overlaps(&claimed_areas, &all_overlapping_points);

    Ok(())
}

fn get_matches<'a>(lines: &'a Vec<String>) -> Vec<regex::Captures> {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    lines
        .iter()
        .map(|line| re.captures(line).unwrap())
        .collect()
}

fn get_areas(matches: &Vec<regex::Captures>) -> Vec<ClaimedArea> {
    matches
        .iter()
        .map(|r_match| ClaimedArea::new(r_match).unwrap())
        .collect()
}

fn get_overlapping_points<'a>(claimed_areas: &'a Vec<ClaimedArea>) -> HashSet<&'a (u32, u32)> {
    let mut all_overlapping_points: HashSet<&(u32, u32)> = HashSet::new();
    let mut read = 1;

    let len = claimed_areas.len();

    for x in claimed_areas.iter().take(len - 1) {
        for y in claimed_areas.iter().skip(read) {
            if x.overlaps(y) {
                let overlapping_points = x.overlapping_points(y);
                for point in &overlapping_points {
                    all_overlapping_points.insert(point);
                }
            }
        }
        read += 1;
    }
    all_overlapping_points
}

fn areas_no_overlaps(
    claimed_areas: &Vec<ClaimedArea>,
    all_overlapping_points: &HashSet<&(u32, u32)>,
) {
    for claimed_area in claimed_areas {
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
}
// #15 @ 916,559: 29x27
