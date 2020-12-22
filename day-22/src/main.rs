use std::collections::VecDeque;

static INPUT: &str = include_str!("../input.txt");
// static INPUT: &str = include_str!("../test-input.txt");

fn main() {
    part1();
}

fn part1() {
    let mut decks = INPUT.split("\n\n");
    let mut deck1: VecDeque<usize> = build_deck(decks.next().unwrap());
    let mut deck2: VecDeque<usize> = build_deck(decks.next().unwrap());

    while !deck1.is_empty() && !deck2.is_empty() {
        let p1 = deck1.pop_front().unwrap();
        let p2 = deck2.pop_front().unwrap();

        if p1 < p2 {
            deck2.push_back(p2);
            deck2.push_back(p1);
        } else {
            // p1 > p2 (can't be equal)
            deck1.push_back(p1);
            deck1.push_back(p2);
        }
    }

    let score = if deck1.is_empty() {
        compute_score(&deck2)
    } else {
        compute_score(&deck1)
    };

    println!("part1 = {}", score);
}

fn build_deck(cards: &str) -> VecDeque<usize> {
    cards
        .lines()
        .skip(1)
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn compute_score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * card)
        .sum()
}
