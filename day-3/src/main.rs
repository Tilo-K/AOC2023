use std::{fs, rc::Rc, sync::Mutex};

fn get_data_array(lines: Vec<&str>) -> Vec<Vec<char>> {
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        data.push(row);
    }
    return data;
}

fn get_machine_numbers(data: Vec<Vec<char>>) -> Vec<u32> {
    let mut machine_numbers: Vec<u32> = Vec::new();
    for (x, row) in data.iter().enumerate() {
        let mut current_number: String = String::new();
        let mut has_adjacent_symbol: bool = false;

        for (y, c) in row.iter().enumerate() {
            if !c.to_owned().is_digit(10) {
                if current_number.len() > 0 && has_adjacent_symbol {
                    println!("Found machine number: {}", current_number);

                    machine_numbers.push(current_number.parse::<u32>().unwrap());
                    has_adjacent_symbol = false;
                }
                current_number = String::new();
            }

            if c.is_digit(10) {
                current_number += c.to_string().as_str();
                let mut top = '.';
                let mut bottom = '.';
                let mut left = '.';
                let mut right = '.';
                let mut top_left = '.';
                let mut top_right = '.';
                let mut bottom_left = '.';
                let mut bottom_right = '.';

                if y != 0 {
                    top = data
                        .get(x)
                        .unwrap_or(&vec![])
                        .get(y - 1)
                        .unwrap_or(&'.')
                        .clone();

                    top_right = data
                        .get(x + 1)
                        .unwrap_or(&vec![])
                        .get(y - 1)
                        .unwrap_or(&'.')
                        .clone();

                    if x != 0 {
                        top_left = data
                            .get(x - 1)
                            .unwrap_or(&vec![])
                            .get(y - 1)
                            .unwrap_or(&'.')
                            .clone();
                    }
                }

                if x != 0 {
                    left = data
                        .get(x - 1)
                        .unwrap_or(&vec![])
                        .get(y)
                        .unwrap_or(&'.')
                        .clone();

                    bottom_left = data
                        .get(x - 1)
                        .unwrap_or(&vec![])
                        .get(y + 1)
                        .unwrap_or(&'.')
                        .clone();
                }

                bottom_right = data
                    .get(x + 1)
                    .unwrap_or(&vec![])
                    .get(y + 1)
                    .unwrap_or(&'.')
                    .clone();

                bottom = data
                    .get(x)
                    .unwrap_or(&vec![])
                    .get(y + 1)
                    .unwrap_or(&'.')
                    .clone();
                right = data
                    .get(x + 1)
                    .unwrap_or(&vec![])
                    .get(y)
                    .unwrap_or(&'.')
                    .clone();

                let adj_places = vec![
                    top,
                    bottom,
                    left,
                    right,
                    top_left,
                    top_right,
                    bottom_left,
                    bottom_right,
                ];

                for place in adj_places {
                    if place != '.' && !place.is_digit(10) {
                        println!("{} is adjacent to {}", place, c);
                        has_adjacent_symbol = true;
                    }
                }
            }

            if y == row.len() - 1 && current_number.len() > 0 && has_adjacent_symbol {
                println!("Found machine number: {}", current_number);
                machine_numbers.push(current_number.parse::<u32>().unwrap());
                has_adjacent_symbol = false;
            }
        }
    }
    return machine_numbers;
}

struct Gear {
    x: usize,
    y: usize,
    sizes: Mutex<Vec<u32>>,
}

fn get_gear_ratios(data: Vec<Vec<char>>) -> Vec<u32> {
    let mut gear_ratios: Vec<u32> = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();

    for (x, row) in data.iter().enumerate() {
        let mut current_number: String = String::new();
        let mut has_adjacent_symbol: bool = false;
        let mut adjacent_x = 0;
        let mut adjacent_y = 0;

        for (y, c) in row.iter().enumerate() {
            if !c.to_owned().is_digit(10) {
                if current_number.len() > 0 && has_adjacent_symbol {
                    println!("Found gear number: {}", current_number);
                    let num = current_number.parse::<u32>().unwrap();

                    let default = Gear {
                        x: adjacent_x,
                        y: adjacent_y,
                        sizes: Mutex::new(vec![num]),
                    };

                    let mut gear = gears
                        .iter()
                        .find(|&g| g.x == adjacent_x && g.y == adjacent_y);


                    if gear.is_none() {
                        gears.push(default);
                    } else {
                        gear.unwrap().sizes.lock().unwrap().push(num);
                    }

                    has_adjacent_symbol = false;
                }
                current_number = String::new();
            }

            if c.is_digit(10) {
                current_number += c.to_string().as_str();
                let mut top = '.';
                let mut bottom = '.';
                let mut left = '.';
                let mut right = '.';
                let mut top_left = '.';
                let mut top_right = '.';
                let mut bottom_left = '.';
                let mut bottom_right = '.';

                if y != 0 {
                    top = data
                        .get(x)
                        .unwrap_or(&vec![])
                        .get(y - 1)
                        .unwrap_or(&'.')
                        .clone();
                    if top == '*' {
                        adjacent_x = x;
                        adjacent_y = y - 1;
                    }

                    top_right = data
                        .get(x + 1)
                        .unwrap_or(&vec![])
                        .get(y - 1)
                        .unwrap_or(&'.')
                        .clone();

                    if top_right == '*' {
                        adjacent_x = x + 1;
                        adjacent_y = y - 1;
                    }
                    if x != 0 {
                        top_left = data
                            .get(x - 1)
                            .unwrap_or(&vec![])
                            .get(y - 1)
                            .unwrap_or(&'.')
                            .clone();
                        if top_left == '*' {
                            adjacent_x = x - 1;
                            adjacent_y = y - 1;
                        }
                    }
                }

                if x != 0 {
                    left = data
                        .get(x - 1)
                        .unwrap_or(&vec![])
                        .get(y)
                        .unwrap_or(&'.')
                        .clone();

                    if left == '*' {
                        adjacent_x = x - 1;
                        adjacent_y = y;
                    }

                    bottom_left = data
                        .get(x - 1)
                        .unwrap_or(&vec![])
                        .get(y + 1)
                        .unwrap_or(&'.')
                        .clone();

                    if bottom_left == '*' {
                        adjacent_x = x - 1;
                        adjacent_y = y + 1;
                    }
                }

                bottom_right = data
                    .get(x + 1)
                    .unwrap_or(&vec![])
                    .get(y + 1)
                    .unwrap_or(&'.')
                    .clone();

                if bottom_right == '*' {
                    adjacent_x = x + 1;
                    adjacent_y = y + 1;
                }

                bottom = data
                    .get(x)
                    .unwrap_or(&vec![])
                    .get(y + 1)
                    .unwrap_or(&'.')
                    .clone();

                if bottom == '*' {
                    adjacent_x = x;
                    adjacent_y = y + 1;
                }

                right = data
                    .get(x + 1)
                    .unwrap_or(&vec![])
                    .get(y)
                    .unwrap_or(&'.')
                    .clone();

                if right == '*' {
                    adjacent_x = x + 1;
                    adjacent_y = y;
                }

                let adj_places = vec![
                    top,
                    bottom,
                    left,
                    right,
                    top_left,
                    top_right,
                    bottom_left,
                    bottom_right,
                ];

                for place in adj_places {
                    if place == '*' {
                        println!("{} is adjacent to {}", place, c);
                        has_adjacent_symbol = true;
                    }
                }
            }

            if y == row.len() - 1 && current_number.len() > 0 && has_adjacent_symbol {
                println!("Found gear number: {}", current_number);
                let num = current_number.parse::<u32>().unwrap();

                let default = Gear {
                    x: adjacent_x,
                    y: adjacent_y,
                    sizes: Mutex::new(vec![num]),
                };

                let mut gear = gears
                    .iter()
                    .find(|&g| g.x == adjacent_x && g.y == adjacent_y);


                if gear.is_none() {
                    gears.push(default);
                } else {
                    gear.unwrap().sizes.lock().unwrap().push(num);
                }

                has_adjacent_symbol = false;
            }
        }
    }

    for gear in gears {
        let sizes = gear.sizes.lock().unwrap();
        if sizes.len() != 2{
            continue;
        }
        
        let ratio = sizes[0] * sizes[1];
        gear_ratios.push(ratio);
    }

    return gear_ratios;
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.lines().collect();
    let data_array = get_data_array(lines);
    let machine_numbers = get_machine_numbers(data_array.clone());
    let gear_ratios = get_gear_ratios(data_array);

    let sum = machine_numbers.iter().sum::<u32>();
    println!("Sum: {}", sum);

    let sumB = gear_ratios.iter().sum::<u32>();
    println!("SumB: {}", sumB);
}
