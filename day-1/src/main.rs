use std::{fs, fmt::format};

fn get_digits(lines: Vec<&str>) -> Vec<Vec<u32>>{
    let mut digits = Vec::new();
    for line in lines {
        let lowercase = line.to_lowercase();
        let mut digs = Vec::new();

        for c in lowercase.chars() {
            if "0123456789".contains(c) {
                let v = c.to_digit(10);
                if v.is_none() {
                    continue;
                }
                digs.push(v.unwrap());
            }
        }

        digits.push(digs);
    }

    digits
}

fn sum_first_last(digits: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;

    for digit in digits {
        let first = digit.first();
        let last = digit.last();

        if first.is_none() || last.is_none() {
            continue;
        }

        let new_number = format!("{}{}", first.unwrap(), last.unwrap());
        dbg!(&new_number);
        sum  += new_number.parse::<u32>().unwrap();
    }

    return sum;
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Error reading input");
    let lines = data
        .split("\n")
        .collect::<Vec<&str>>();
       
    let digits = get_digits(lines.clone());
    let sumA = sum_first_last(digits);
    let mut replaced_lines = Vec::new();
    for line in lines {
        let mut new_line = String::new();
        
        new_line = line.replace("one", "one1one");
        new_line = new_line.replace("two", "two2two");
        new_line = new_line.replace("three", "three3three");
        new_line = new_line.replace("four", "four4four");
        new_line = new_line.replace("five", "five5five");
        new_line = new_line.replace("six", "six6six");
        new_line = new_line.replace("seven", "seven7seven");
        new_line = new_line.replace("eight", "eight8eight");
        new_line = new_line.replace("nine", "nine9nine");

        replaced_lines.push(new_line);
    }

    let digitsB = get_digits(replaced_lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
    let sumB = sum_first_last(digitsB);

    println!("{}", sumA);
    println!("{}", sumB);
}
