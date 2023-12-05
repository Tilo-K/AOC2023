use std::{fs, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

struct Range {
    dest_range_start: u64,
    src_range_start: u64,
    range_length: u64,
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        Ok(Range {
            dest_range_start: nums[0],
            src_range_start: nums[1],
            range_length: nums[2],
        })
    }
}

impl Range {
    fn transalte_num(&self, num: u64) -> Option<u64> {
        if self.src_range_start <= num && num < self.src_range_start + self.range_length {
            return Some(self.dest_range_start + (num - self.src_range_start));
        }

        None
    }
}

struct Map {
    ranges: Vec<Range>,
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect::<Vec<&str>>();
        let ranges = lines[1..]
            .iter()
            .filter(|s| !s.trim().is_empty())
            .map(|l| l.parse::<Range>().unwrap())
            .collect::<Vec<Range>>();
        Ok(Map { ranges: ranges })
    }
}

impl Map {
    fn translate_num(&self, num: u64) -> u64 {
        for r in &self.ranges {
            if let Some(n) = r.transalte_num(num) {
                return n;
            }
        }

        num
    }
}

fn part_a(blocks: &Vec<&str>) {
    let (_, seed_num_str) = blocks[0].split_once(":").expect("Unable to split seeds");

    let seeds = seed_num_str
        .trim()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let maps = blocks[1..]
        .iter()
        .map(|b| b.parse::<Map>().unwrap())
        .collect::<Vec<Map>>();

    let mut locations: Vec<u64> = Vec::new();

    for seed in &seeds {
        let mut num = *seed;
        for map in &maps {
            num = map.translate_num(num);
        }
        locations.push(num);
    }

    let min_loc = locations.iter().min().unwrap();

    println!("Part A: {}", min_loc);
}

fn main() {
    let data = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .replace("\r", "");
    let blocks = data.split("\n\n").collect::<Vec<&str>>();
    part_a(&blocks);
}
