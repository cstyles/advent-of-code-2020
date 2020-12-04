use regex::Regex;
use std::collections::HashMap;

lazy_static::lazy_static! (
    static ref RE: Regex = {
        Regex::new(r#"^(\d+)-(\d+) (\w): (.*)$"#).expect("couldn't compile regex")
    };
);

#[derive(Default, Debug, Clone)]
struct Password {
    required_letter: char,
    min: i32,
    max: i32,
    string: String,
}

impl Password {
    fn is_valid_part_1(&self) -> bool {
        let mut hm = HashMap::<char, i32>::new();
        for character in self.string.chars() {
            hm.entry(character).and_modify(|i| *i += 1).or_insert(1);
        }

        match hm.get(&self.required_letter) {
            None => false,
            Some(count) if *count >= self.min && *count <= self.max => true,
            _ => false,
        }
    }
}

impl std::convert::From<&str> for Password {
    fn from(string: &str) -> Self {
        let captures = RE.captures(string).expect("coudn't match regex");

        Self {
            required_letter: captures.get(3).unwrap().as_str().chars().next().unwrap(),
            min: captures.get(1).unwrap().as_str().parse().unwrap(),
            max: captures.get(2).unwrap().as_str().parse().unwrap(),
            string: captures.get(4).unwrap().as_str().to_string(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let count: usize = input
        .trim()
        .split("\n")
        .map(|line| Password::from(line))
        .filter(|p| p.is_valid_part_1())
        .count();

    println!("part1 = {}", count);
}
