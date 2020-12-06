use std::collections::HashSet;

static INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
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
