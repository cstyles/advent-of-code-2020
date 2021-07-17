use std::collections::HashMap;

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let parts: (usize, usize) = INPUT
        .trim()
        .split("\n\n")
        .map(|group| solve_parts(group))
        .fold((0, 0), |(part1_sum, part2_sum), (part1, part2)| {
            (part1_sum + part1, part2_sum + part2)
        });

    println!("part1: {}", parts.0);
    println!("part2: {}", parts.1);
}

fn solve_parts(group: &str) -> (usize, usize) {
    let size_of_group = group.lines().count();

    let answers = group
        .lines()
        .fold(HashMap::new(), |mut answers, persons_answers| {
            persons_answers.chars().for_each(|answer| {
                answers.entry(answer).and_modify(|x| *x += 1).or_insert(1);
            });

            answers
        });

    let part1 = answers.len();

    let part2 = answers
        .iter()
        .filter(|(_character, count)| **count == size_of_group)
        .count();

    (part1, part2)
}
