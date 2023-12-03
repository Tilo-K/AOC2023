use std::{fs, cmp};

struct Game{
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

fn get_games(lines: Vec<&str>) -> Vec<Game>{
    let mut games: Vec<Game> = Vec::new();

    for line in lines{
        if line.trim() == ""{
            continue;
        }

        let mut game = Game{ id: 0, red: 0, green: 0, blue: 0};
        let parts = line.split(":").collect::<Vec<&str>>();
        let first = parts.first().unwrap_or(&"");
        if !first.starts_with("Game"){
            println!("Invalid line: {}", line);
            continue;
        }

        let game_splits = first.split(" ").collect::<Vec<&str>>();
        let id = game_splits.last().unwrap_or(&"");

        if id.to_owned() == ""{
            println!("Invalid line: {}", line);
            continue;
        }

        game.id = id.parse::<u32>().unwrap_or(0);
        let sets = parts.last().unwrap_or(&"").split(";").collect::<Vec<&str>>();
        for set in sets{
            let colors = set.trim().split(",").collect::<Vec<&str>>();
            for color in colors{
                let halfs: Vec<&str> = color.trim().split(" ").collect();
                if halfs.len() != 2{
                    println!("Invalid color: {}", color);
                    continue;
                }

                let count: u32 = halfs.first().unwrap_or(&"").trim().parse().unwrap_or(0);
                let color = halfs.last().unwrap_or(&"").trim();

                match color{
                    "red" => game.red = cmp::max(game.red, count),
                    "green" => game.green = cmp::max(game.green, count),
                    "blue" => game.blue = cmp::max(game.blue, count),
                    _ => println!("Invalid color: {}", color),
                }
            }
        }

        games.push(game);
    }

    return games;
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut lines: Vec<&str> = data.lines().collect();
    let games = get_games(lines);

    let mut id_sum = 0;
    let mut power_sum = 0;
    for game in games{
        println!("Game: {}", game.id);
        println!("Red: {}", game.red);
        println!("Green: {}", game.green);
        println!("Blue: {}", game.blue);
        println!("");

        if game.red <= 12 && game.green <= 13 && game.blue <= 14{
            id_sum += game.id;
        }

        power_sum += game.red * game.green * game.blue;
    }

    println!("Sum of IDs: {}", id_sum);
    println!("Sum of Power: {}", power_sum);
}
