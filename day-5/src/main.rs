use std::{collections::HashSet, fs, str::FromStr, sync::{Mutex, Arc}};

#[derive(Debug, PartialEq, Eq)]
struct ParseError;
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

struct MinLoc {
    num: u64,
}

fn part_b(blocks: &Vec<&str>) {
    let (_, seed_num_str) = blocks[0].split_once(":").expect("Unable to split seeds");

    let seeds = seed_num_str
        .trim()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let orig_maps = blocks[1..]
        .iter()
        .map(|b| b.parse::<Map>().unwrap())
        .collect::<Vec<Map>>();

    let min_lock = Arc::new(Mutex::new(MinLoc { num: u64::MAX }));
    let mut handles = vec![];

    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i];
        let length = seeds[i + 1].clone();
        let maps = orig_maps.clone();
        let min_loc = Arc::clone(&min_lock);

        let handle = std::thread::spawn(move || {
            let mut min = u64::MAX;

            for j in start..start + length {
                let mut num = j;
                for map in &maps {
                    num = map.translate_num(num);
                }

                if num < min {
                    min = num;
                }
            }

            let mut min_loc = min_loc.lock().unwrap();
            if min < min_loc.num {
                min_loc.num = min;
            }
            println!("Finished {} to {}", start, start + length);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Part B: {}", min_lock.lock().unwrap().num);
}

fn main() {
    let data = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .replace("\r", "");
    let blocks = data.split("\n\n").collect::<Vec<&str>>();
    part_a(&blocks);
    part_b(&blocks);
}
