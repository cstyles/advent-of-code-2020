#[derive(Debug)]
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

impl<'a> From<&'a str> for Password<'a> {
    fn from(string: &'a str) -> Self {
        let (min, rest) = string.split_once('-').unwrap();
        let (max, rest) = rest.split_once(' ').unwrap();
        let (required_letter, string) = rest.split_once(": ").unwrap();

        Self {
            required_letter: required_letter.chars().next().unwrap(),
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
            string,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let passwords: Vec<Password> = input.lines().map(Password::from).collect();

    part1(&passwords);
    part2(&passwords);
}

fn part1(passwords: &[Password]) {
    let count: usize = passwords.iter().filter(|p| p.is_valid_part_1()).count();

    println!("part1 = {}", count);
}

fn part2(passwords: &[Password]) {
    let count: usize = passwords.iter().filter(|p| p.is_valid_part_2()).count();

    println!("part2 = {}", count);
}
