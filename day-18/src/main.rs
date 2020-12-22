static INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

fn main() {
    part1();
}

fn part1() {
    let sum: u64 = INPUT
        .lines()
        .map(|line| line.chars().filter(|c| *c != ' ').collect::<Vec<char>>())
        .map(|line| eval_expr(&line))
        .sum();

    println!("part1 = {}", sum);
}

fn eval_expr(string: &[char]) -> u64 {
    use Operation::*;

    let mut operation = Add;
    let mut value = 0;

    let mut i = 0;
    while i < string.len() {
        match string[i] {
            c @ '0'..='9' => {
                let operand = c.to_digit(10).unwrap() as u64;

                match operation {
                    Add => value += operand,
                    Multiply => value *= operand,
                };
            }
            '+' => operation = Add,
            '*' => operation = Multiply,
            '(' => {
                let close_paren_index = find_close_paren(&string, i);
                let sub_expr = &string[i + 1..close_paren_index];
                let operand = eval_expr(sub_expr);

                // TODO: DRY
                match operation {
                    Add => value += operand,
                    Multiply => value *= operand,
                };

                i = close_paren_index;
            }
            _ => (),
        };

        i += 1;
    }

    value
}

fn find_close_paren(string: &[char], start: usize) -> usize {
    let mut depth = 0;

    for (i, c) in string.iter().enumerate().skip(start + 1) {
        match (c, depth) {
            ('(', _) => depth += 1,
            (')', 0) => return i,
            (')', _) => depth -= 1,
            _ => (),
        }
    }

    unreachable!();
}
