use std::{
    cmp::Ordering, collections::HashMap, error::Error, fs, str::FromStr, string::ParseError, usize,
};

#[derive(Clone, PartialEq, PartialOrd, Eq, Debug)]
enum Card {
    A = 13,
    K = 12,
    Q = 11,
    J = 10,
    T = 9,
    N9 = 8,
    N8 = 7,
    N7 = 6,
    N6 = 5,
    N5 = 4,
    N4 = 3,
    N3 = 2,
    N2 = 1,
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u128,
}

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(" ").unwrap();
        let bid = bid_str.parse::<u128>();
        let mut cards = vec![];

        for card in cards_str.chars() {
            let c = match card {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::J,
                'T' => Card::T,
                '9' => Card::N9,
                '8' => Card::N8,
                '7' => Card::N7,
                '6' => Card::N6,
                '5' => Card::N5,
                '4' => Card::N4,
                '3' => Card::N3,
                '2' => Card::N2,
                _ => continue,
            };

            cards.push(c);
        }

        let res = Hand {
            bid: bid.unwrap(),
            cards,
        };

        Ok(res)
    }
}

impl Hand {
    fn get_power(&self) -> u16 {
        let mut five_of_a_kind = true;
        for i in 1..self.cards.len() {
            if self.cards[i - 1].clone() as u8 != self.cards[i].clone() as u8 {
                five_of_a_kind = false;
                break;
            }
        }

        if five_of_a_kind {
            return 7;
        }

        let mut occ: HashMap<u8, usize> = HashMap::new();
        for x in self.cards.clone() {
            *occ.entry(x as u8).or_default() += 1;
        }

        let mut has_three = false;
        let mut twos = 0;

        for val in occ.values() {
            if *val == 4 as usize {
                return 6;
            }

            if *val == 3 as usize {
                has_three = true;
            }

            if *val == 2 as usize {
                twos += 1;
            }
        }

        if twos == 1 && has_three {
            return 5;
        }

        if has_three {
            return 4;
        }

        if twos == 2 {
            return 3;
        }

        if twos == 1 {
            return 2;
        }

        return 1;
    }

    fn compare(&self, comp: &Hand) -> i32 {
        if self.get_power() != comp.get_power() {
            return (self.get_power() - comp.get_power()).into();
        }

        for i in 0..self.cards.len() {
            let val_a = self.cards[i].clone() as u8;
            let val_b = comp.cards[i].clone() as u8;

            if val_a == val_b {
                continue;
            }

            if val_a > val_b {
                return 1;
            }

            return -1;
        }

        return 0;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.compare(other);
        if cmp == 0 {
            return Ordering::Equal;
        }

        if cmp > 0 {
            return Ordering::Greater;
        }

        return Ordering::Less;
    }
}

fn main() {
    let lines = fs::read_to_string("example.txt")
        .expect("Cannot read file")
        .split("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let mut hands = lines
        .iter()
        .filter(|s| !s.trim().is_empty())
        .map(|l| Hand::from_str(l).unwrap())
        .collect::<Vec<Hand>>();

    hands.sort();

    let mut sum = 0;
    for i in 0..hands.len() {
        let rank = (i + 1);
        let hand = &hands[i];
        sum += hand.bid * rank as u128;

        println!("{} * {}", hand.bid, rank);
    }

    println!("Total winnings {}", sum);
}
