use std::collections::HashMap;
use std::convert::From;

static INPUT: &str = include_str!("../input.txt");
// static INPUT: &str = include_str!("../test-input.txt");

#[derive(Debug)]
enum Rule<'a> {
    Literal(&'a str),                      // 0: "a"
    Alias(&'a str),                        // 0: 1
    JustSeq(Sequence<'a>),                 // 0: 1 2
    Either(&'a str, &'a str),              // 0: 1 | 2
    EitherSeq(Sequence<'a>, Sequence<'a>), // 0: 1 2 | 3 4
}

#[derive(Debug)]
struct Sequence<'a> {
    first: &'a str,
    second: &'a str,
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(string: &'a str) -> Self {
        let string = string.trim();

        if string == "\"a\"" {
            Rule::Literal("a")
        } else if string == "\"b\"" {
            Rule::Literal("b")
        } else if string.contains('|') {
            let mut bar_splits = string.split('|');

            let left = bar_splits.next().unwrap().trim();

            if left.contains(' ') {
                let mut space_splits = left.split(' ');
                let first = space_splits.next().unwrap();
                let second = space_splits.next().unwrap();
                let left_sequence = Sequence { first, second };

                let right = bar_splits.next().unwrap().trim();
                let mut space_splits = right.split(' ');
                let first = space_splits.next().unwrap();
                let second = space_splits.next().unwrap();
                let right_sequence = Sequence { first, second };

                Rule::EitherSeq(left_sequence, right_sequence)
            } else {
                let right = bar_splits.next().unwrap().trim();

                Rule::Either(left, right)
            }
        } else if string.contains(' ') {
            let mut space_splits = string.split(' ');
            let first = space_splits.next().unwrap();
            let second = space_splits.next().unwrap();
            let sequence = Sequence { first, second };

            Rule::JustSeq(sequence)
        } else {
            Rule::Alias(string)
        }
    }
}

fn main() {
    part1();
}

fn part1() {
    let mut sections = INPUT.split("\n\n");
    let rule_section = sections.next().unwrap();
    let string_section = sections.next().unwrap();

    let rules: HashMap<&str, Rule> = rule_section
        .lines()
        .map(|line| {
            let mut splits = line.split(": ");
            let rule_number = splits.next().unwrap();
            let rule = splits.next().unwrap().into();

            (rule_number, rule)
        })
        .collect();

    let part1 = string_section
        .lines()
        .filter_map(|line| parse(&rules, "0", line).ok())
        .filter(|(rest, _parsed)| rest.is_empty())
        .count();

    println!("part1 = {}", part1);
}

fn parse<'a>(
    rules: &HashMap<&'a str, Rule<'a>>,
    rule_number: &str,
    string: &'a str,
) -> Result<(&'a str, &'a str), ()> {
    use Rule::*;

    match rules.get(rule_number).unwrap() {
        Literal(literal) => {
            if string.starts_with(*literal) {
                let rest = &string[literal.len()..];
                let parsed = literal;

                Ok((rest, parsed))
            } else {
                Err(())
            }
        }
        Alias(alias) => parse(rules, alias, string),
        JustSeq(Sequence { first, second }) => {
            parse(rules, first, string).and_then(|(rest, _parsed)| parse(rules, second, rest))
        }
        Either(left, right) => match parse(rules, left, string) {
            result @ Ok(_) => result,
            Err(_) => parse(rules, right, string),
        },
        EitherSeq(left, right) => {
            let Sequence { first, second } = left;
            let left_result =
                parse(rules, first, string).and_then(|(rest, _parsed)| parse(rules, second, rest));

            if left_result.is_ok() {
                return left_result;
            }

            let Sequence { first, second } = right;
            parse(rules, first, string).and_then(|(rest, _parsed)| parse(rules, second, rest))
        }
    }
}
