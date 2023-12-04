use std::fs;

#[derive(Debug, Clone)]
struct Card {
    id: i32,
    winning: Vec<i32>,
    numbers: Vec<i32>,
}

fn create_card(line: &str) -> Card {
    let parts = line.split(":").collect::<Vec<&str>>();

    let left = parts.first();
    if left.is_none() {
        panic!("Invalid line {}", line);
    }

    let id = left
        .unwrap()
        .trim()
        .split(" ")
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let right = parts.last();
    if right.is_none() {
        panic!("Invalid line {}", line);
    }

    let nums = right.unwrap().trim().split("|").collect::<Vec<&str>>();
    if nums.len() != 2 {
        panic!("Invalid line {}", line);
    }

    let winning = nums[0]
        .trim()
        .split(" ")
        .filter(|s| !s.trim().is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let numbers = nums[1]
        .trim()
        .split(" ")
        .filter(|s| !s.trim().is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    return Card {
        id: id,
        winning: winning,
        numbers: numbers,
    };
}

fn create_cards(lines: Vec<&str>) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let card = create_card(line);
        cards.push(card);
    }

    return cards;
}

fn get_points_for_card(card: &Card) -> i32 {
    let mut points = 0;
    for num in &card.numbers {
        if card.winning.contains(num) {
            if points < 1 {
                points = 1;
                continue;
            }

            points *= 2;
        }
    }

    return points;
}

fn get_winning_in_card(card: &Card) -> Vec<i32> {
    let mut winning: Vec<i32> = Vec::new();
    for num in &card.numbers {
        if card.winning.contains(num) {
            winning.push(num.clone());
        }
    }

    return winning;
}

fn card_with_id(cards: &Vec<Card>, id: i32) -> Card {
    cards.iter().find(|c| c.id == id).unwrap().clone()
}

fn cards_with_id(cards: &Vec<Card>, id: i32) -> Vec<Card> {
    cards
        .iter()
        .filter(|c| c.id == id)
        .map(|c| c.clone())
        .collect::<Vec<Card>>()
        .clone()
}

fn expand_cards(cards: &Vec<Card>) -> Vec<Card> {
    let mut new_cards: Vec<Card> = cards.clone();
    let max_id = cards.iter().max_by_key(|c| c.id).unwrap().id;

    for i in 1..max_id {
        println!("Expanding {}", i);
        let cards_with_id = cards_with_id(&new_cards, i);
        for card in cards_with_id {
            let winning = get_winning_in_card(&card).len();

            for i in (card.id + 1)..(card.id + winning as i32 + 1) {
                let c = card_with_id(cards, i);
                new_cards.push(c);
            }
        }
    }

    return new_cards;
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines: Vec<&str> = data.lines().collect();
    let cards = create_cards(lines);

    let mut sum = 0;
    for card in &cards {
        let points = get_points_for_card(&card);
        sum += points;
    }

    let expanded = expand_cards(&cards);

    println!("Total points: {}", sum);
    println!("Total expanded: {}", expanded.len());
}
