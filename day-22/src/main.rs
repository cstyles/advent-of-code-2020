use std::collections::{HashSet, VecDeque};

static INPUT: &str = include_str!("../input.txt");
// static INPUT: &str = include_str!("../test-input.txt");

fn main() {
    part1();
    part2();
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

fn part2() {
    let mut decks = INPUT.split("\n\n");
    let deck1: VecDeque<usize> = build_deck(decks.next().unwrap());
    let deck2: VecDeque<usize> = build_deck(decks.next().unwrap());

    // let mut game_number = 1;
    let (_, score) = play_game(&mut 0, deck1, deck2);

    println!("part2 = {}", score);
}

// return value == true if player 1 won; usize = score
fn play_game(
    game_number: &mut usize,
    mut deck1: VecDeque<usize>,
    mut deck2: VecDeque<usize>,
) -> (bool, usize) {
    *game_number += 1;
    let mut seen_rounds: HashSet<(Vec<usize>, Vec<usize>)> = Default::default();

    while !deck1.is_empty() && !deck2.is_empty() {
        let round_key: (Vec<usize>, Vec<usize>) = (
            deck1.iter().copied().collect(),
            deck2.iter().copied().collect(),
        );

        if seen_rounds.contains(&round_key) {
            return (true, compute_score(&deck1));
        } else {
            seen_rounds.insert(round_key);
        }

        // New configuration; players draw a card
        let p1 = deck1.pop_front().unwrap();
        let p2 = deck2.pop_front().unwrap();

        // Play Recursive Combat
        if p1 <= deck1.len() && p2 <= deck2.len() {
            let new_deck1: VecDeque<usize> = deck1.iter().copied().take(p1).collect();
            let new_deck2: VecDeque<usize> = deck2.iter().copied().take(p2).collect();

            let (player_1_won, _) = play_game(game_number, new_deck1, new_deck2);

            if player_1_won {
                deck1.push_back(p1);
                deck1.push_back(p2);
            } else {
                deck2.push_back(p2);
                deck2.push_back(p1);
            }
        } else {
            // Regular combat
            if p1 < p2 {
                deck2.push_back(p2);
                deck2.push_back(p1);
            } else {
                deck1.push_back(p1);
                deck1.push_back(p2);
            }
        }
    }

    if deck1.is_empty() {
        (false, compute_score(&deck2))
    } else {
        (true, compute_score(&deck1))
    }
}
