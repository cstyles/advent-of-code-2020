static INPUT: &str = include_str!("../input.txt");

fn main() {
    let numbers: Vec<i64> = INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let invalid_number = part1(&numbers);
    println!("part1 = {}", invalid_number);
}

fn part1(numbers: &[i64]) -> i64 {
    let mut start = 0;
    let mut end = 25;

    while is_valid(&numbers, start, end) {
        start += 1;
        end += 1;
    }

    numbers[end]
}

fn is_valid(numbers: &[i64], start: usize, end: usize) -> bool {
    for i in start..end {
        for j in 0..i {
            if numbers[i] + numbers[j] == numbers[end] {
                return true;
            }
        }
    }

    false
}
