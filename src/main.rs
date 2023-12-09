// 2023 day 4

//#[derive(Debug)]
#[allow(dead_code)]
struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

fn main() {
    part1();
    //part2();
}

fn part1() {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();
    let cards: Vec<Card> = lines.iter().map(|line| parse_card(line)).collect();

    let mut sum: u32 = 0;
    for card in cards.iter() {
        sum += score_card(card);
    }
    println!("sum: {}", sum);
}

fn score_card(card: &Card) -> u32 {
    let mut score = 0;
    for my_number in card.my_numbers.iter() {
        if card.winning_numbers.contains(my_number) {
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
    }

    score
}

fn parse_card(line: &str) -> Card {
    let mut parts = line.split(':');
    let number = parts
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut numbers_iter = parts.next().unwrap().split('|');

    let winning_numbers = parse_numbers(numbers_iter.next().unwrap());
    let my_numbers = parse_numbers(numbers_iter.next().unwrap());

    Card {
        number,
        winning_numbers,
        my_numbers,
    }
}

fn parse_numbers(num_str: &str) -> Vec<u32> {
    num_str
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Failed to parse number"))
        .collect()
}
