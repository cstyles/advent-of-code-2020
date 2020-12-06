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
        .map(|answers| unique_chars(&answers))
        .sum();

    println!("part1: {}", sum);
}

fn unique_chars(string: &str) -> usize {
    string
        .chars()
        .fold(HashSet::new(), |mut chars, x| {
            chars.insert(x);
            chars
        })
        .len()
}

fn part2() {
    let sum: usize = INPUT
        .trim()
        .split("\n\n")
        .map(|group| count_unanimities(group))
        .sum();

    println!("part2: {}", sum);
}

fn count_unanimities(group: &str) -> usize {
    let size_of_group = group.lines().count();

    let answers = group
        .lines()
        .fold(HashMap::new(), |mut answers, persons_answers| {
            persons_answers.chars().for_each(|answer| {
                answers.entry(answer).and_modify(|x| *x += 1).or_insert(1);
            });

            answers
        });

    answers
        .iter()
        .filter(|(_character, count)| **count == size_of_group)
        .count()
}
