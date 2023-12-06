use std::fs;

fn win_possibilities(hold: u128, dist: u128) -> u128{
    let mut wins = 0;
    for i in 0..hold {
        if (hold-i) * i > dist {
            wins += 1;
        }
    }
    wins
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.split("\n").collect();

    let (_,times) = lines[0].split_once(":").unwrap();
    let race_times: Vec<u128> = times.split_ascii_whitespace().map(|x| x.parse::<u128>().unwrap()).collect();

    let (_,distances) = lines[1].split_once(":").unwrap();
    let race_dists: Vec<u128> = distances.split_ascii_whitespace().map(|x| x.parse::<u128>().unwrap()).collect();

    let mut margin = 0;
    for i in 0..race_times.len() {
        let wins = win_possibilities(race_times[i], race_dists[i]);
        if margin == 0{
            margin = wins;
        } else {
            margin *= wins;
        }
    }

    println!("The margin is {}", margin);

    let big_hold = times.replace(" ", "").trim().parse::<u128>().unwrap();
    let big_dist = distances.replace(" ", "").trim().parse::<u128>().unwrap();

    let big_margin = win_possibilities(big_hold, big_dist);
    println!("The big margin is {}", big_margin);
}
