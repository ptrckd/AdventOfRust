extern crate regex;
use regex::Regex;

use std::collections::{HashMap, HashSet};
use std::error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct ClaimedArea {
    id: u32,
    south_edge: i32,
    west_edge: i32,
    width: i32,
    height: i32,
    claimed_points: HashSet<(i32, i32)>,
}

impl ClaimedArea {
    //west, south, width, height
    fn new(captures: &regex::Captures) -> Result<ClaimedArea, std::num::ParseIntError> {
        let id = captures.get(1).unwrap().as_str().parse()?;
        let south_edge = captures.get(3).unwrap().as_str().parse()?;
        let west_edge = captures.get(2).unwrap().as_str().parse()?;
        let width = captures.get(4).unwrap().as_str().parse()?;
        let height = captures.get(5).unwrap().as_str().parse()?;

        let mut claimed_points: HashSet<(i32, i32)> = HashSet::new();

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
        !(self.west_edge > other.west_edge + other.width
            || self.west_edge + self.width < other.west_edge
            || self.south_edge + self.height < other.south_edge
            || self.south_edge > other.south_edge + other.height)
    }

    fn overlapping_points(&self, other: &ClaimedArea) -> HashSet<(i32, i32)> {
        self.claimed_points
            .intersection(&other.claimed_points)
            .cloned()
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

    let grid = create_grid(&claimed_areas);

    println!(
        "Total overlapping points: {}",
        grid.values().filter(|x| **x != 1).count()
    );

    areas_no_overlaps(&claimed_areas, &grid);

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

fn create_grid(claimed_areas: &Vec<ClaimedArea>) -> HashMap<(i32, i32), u32> {
    let mut grid: HashMap<(i32, i32), u32> = HashMap::new();

    for area in claimed_areas {
        for (s, w) in area.claimed_points.iter() {
            *grid.entry((*s, *w)).or_insert(0) += 1;
        }
    }
    grid
}

fn areas_no_overlaps(claimed_areas: &Vec<ClaimedArea>, grid: &HashMap<(i32, i32), u32>) {
    for claimed_area in claimed_areas {
        if claimed_area
            .claimed_points
            .iter()
            .all(|(s, w)| grid[&(*s, *w)] == 1)
        {
            println!("No overlap on area: {}", claimed_area.id);
        }
    }
}
// #15 @ 916,559: 29x27
