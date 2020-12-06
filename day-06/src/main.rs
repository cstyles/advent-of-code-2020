use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let sum: usize = INPUT
        .trim()
        .split("\n\n")
        .map(|group| group.lines().collect::<String>())
        .map(|answers| count_chars(&answers))
        .sum();

    println!("part1: {}", sum);
}

fn count_chars(string: &str) -> usize {
    let chars = string
        .chars()
        .fold(HashSet::new(), |mut acc, x| {
            acc.insert(x);
            acc
        });

    chars.len()
}

fn part2() {
    let sum: usize = INPUT
        .trim()
        .split("\n\n")
        .map(|group| group.lines().collect::<Vec<&str>>())
        .map(|people| count_unanimities(&people))
        .sum();

    println!("part2: {}", sum);
}

fn count_unanimities(people: &[&str]) -> usize {
    let mut answers: HashMap<char, usize> = Default::default();
    for persons_answers in people {
        for character in persons_answers.chars() {
            answers
                .entry(character)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }

    answers
        .iter()
        .filter(|(_character, count)| **count == people.len())
        .count()
}
