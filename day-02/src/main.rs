use regex::Regex;

static INPUT: &str = include_str!("../input.txt");

lazy_static::lazy_static! (
    static ref RE: Regex = {
        Regex::new(r#"^(\d+)-(\d+) (\w): (.*)$"#).expect("couldn't compile regex")
    };
);

#[derive(Default, Debug, Clone)]
struct Password<'a> {
    required_letter: char,
    min: usize,
    max: usize,
    string: &'a str,
}

impl<'a> Password<'a> {
    fn is_valid_part_1(&self) -> bool {
        let count = self
            .string
            .chars()
            .filter(|c| *c == self.required_letter)
            .count();

        count >= self.min && count <= self.max
    }

    fn is_valid_part_2(&self) -> bool {
        let mut chars = self.string.chars();

        let char1 = chars.nth(self.min - 1).unwrap();
        let char2 = chars.nth(self.max - self.min - 1).unwrap();

        (char1 == self.required_letter) ^ (char2 == self.required_letter)
    }
}

impl<'a> std::convert::From<&'a str> for Password<'a> {
    fn from(string: &'a str) -> Self {
        let captures = RE.captures(string).expect("coudn't match regex");

        Self {
            required_letter: captures.get(3).unwrap().as_str().chars().next().unwrap(),
            min: captures.get(1).unwrap().as_str().parse().unwrap(),
            max: captures.get(2).unwrap().as_str().parse().unwrap(),
            string: captures.get(4).unwrap().as_str(),
        }
    }
}

fn main() {
    let passwords: Vec<Password> = INPUT
        .trim()
        .lines()
        .map(|line| Password::from(line))
        .collect();

    part1(&passwords);
    part2(&passwords);
}

fn part1<'a>(passwords: &'a [Password]) {
    let count: usize = passwords.iter().filter(|p| p.is_valid_part_1()).count();

    println!("part1 = {}", count);
}

fn part2<'a>(passwords: &'a [Password]) {
    let count: usize = passwords.iter().filter(|p| p.is_valid_part_2()).count();

    println!("part2 = {}", count);
}
