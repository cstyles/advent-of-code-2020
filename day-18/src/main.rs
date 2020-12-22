static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Tree {
    left: Box<Node>,
    operation: Operation,
    right: Box<Node>,
}

#[derive(Debug, Clone)]
enum Node {
    Value(u64),
    Tree(Tree),
}

fn main() {
    part1();
    part2();
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

fn part2() {
    let lines: Vec<String> = INPUT.lines().map(|line| line.replace(' ', "")).collect();

    let sum: u64 = lines
        .iter()
        .map(|line| parse_expr(&line))
        .map(|node| eval_tree(node))
        .sum();

    println!("part2 = {}", sum);
}

// Recursively parse an expression into an expression tree
fn parse_expr(string: &str) -> Node {
    if string.len() == 0 {
        panic!("empty expr");
    }

    let (rest, left) = chomp_expr(string);
    if rest.is_empty() {
        return left;
    }

    let (rest, operation) = parse_operation(rest);

    match operation {
        Operation::Multiply => {
            let right = parse_expr(rest);
            let left = Box::new(left);
            let right = Box::new(right);

            Node::Tree(Tree {
                left,
                operation,
                right,
            })
        }
        Operation::Add => {
            let (rest, right) = chomp_expr(rest);
            let left = Box::new(left);
            let right = Box::new(right);

            let sub_tree = Node::Tree(Tree {
                left,
                operation,
                right,
            });

            build_up(sub_tree, rest)
        }
    }
}

// Build an expression tree with `node` as its left child
fn build_up(node: Node, string: &str) -> Node {
    if string.is_empty() {
        return node;
    }

    let (rest, operation) = parse_operation(string);
    let left = Box::new(node);

    match operation {
        Operation::Multiply => {
            let right = parse_expr(rest);
            let right = Box::new(right);

            Node::Tree(Tree {
                left,
                operation,
                right,
            })
        }
        Operation::Add => {
            let (rest, right) = chomp_expr(rest);
            let right = Box::new(right);

            let tree = Node::Tree(Tree {
                left,
                operation,
                right,
            });

            build_up(tree, rest)
        }
    }
}

// Parse just the expression on the left and return it with the rest of the string
fn chomp_expr(string: &str) -> (&str, Node) {
    match string.chars().next().unwrap() {
        c @ '0'..='9' => {
            let value = parse_number(c);
            let rest = &string[1..];
            (rest, value)
        }
        '(' => parse_parenthetical(string),
        _ => panic!("couldn't chomp: {}", string),
    }
}

fn parse_number(c: char) -> Node {
    let value = c.to_digit(10).unwrap() as u64;

    Node::Value(value)
}

fn parse_operation(string: &str) -> (&str, Operation) {
    let rest = &string[1..];
    match string.chars().next().unwrap() {
        '+' => (rest, Operation::Add),
        '*' => (rest, Operation::Multiply),
        op @ _ => panic!("invalid operation: {}", op),
    }
}

fn parse_parenthetical(string: &str) -> (&str, Node) {
    let mut depth = 0;

    for (i, c) in string.chars().enumerate().skip(1) {
        match (c, depth) {
            ('(', _) => depth += 1,
            (')', 0) => {
                let inner = &string[1..i];
                let node = parse_expr(inner);
                let rest = &string[i + 1..];

                return (rest, node);
            }
            (')', _) => depth -= 1,
            _ => (),
        }
    }

    unreachable!();
}

fn eval_tree(node: Node) -> u64 {
    use Operation::*;

    match node {
        Node::Value(val) => val,
        Node::Tree(Tree {
            left,
            operation: Add,
            right,
        }) => eval_tree(*left) + eval_tree(*right),
        Node::Tree(Tree {
            left,
            operation: Multiply,
            right,
        }) => eval_tree(*left) * eval_tree(*right),
    }
}
