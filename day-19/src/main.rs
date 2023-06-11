use std::collections::HashMap;

static INPUT: &str = include_str!("../input-part1.txt");

#[derive(Debug)]
enum Rule<'a> {
    Literal(&'a str),   // 0: "a"
    Alias(u8),          // 0: 1
    Sequence(Vec<u8>),  // 0: 1 2
    Alt(Vec<Rule<'a>>), // 0: 1 | 2
    Final(u8, u8),
}

impl<'a> Rule<'a> {
    fn new(string: &'a str, rule_number: u8, is_part2: bool) -> Self {
        let string = string.trim();

        if string == "\"a\"" {
            Rule::Literal("a")
        } else if string == "\"b\"" {
            Rule::Literal("b")
        } else if rule_number == 0 && is_part2 {
            Rule::Final(42, 31)
        } else if string.contains('|') {
            let alt_parts = string
                .split('|')
                .map(|s| Rule::new(s.trim(), rule_number, is_part2))
                .collect();

            Rule::Alt(alt_parts)
        } else if string.contains(' ') {
            let seq_parts: Vec<u8> = string
                .split(' ')
                .map(|part| part.parse().unwrap())
                .collect();

            Rule::Sequence(seq_parts)
        } else {
            Rule::Alias(string.parse().unwrap())
        }
    }
}

fn get_rules(input: &str, is_part2: bool) -> HashMap<u8, Rule> {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split(": ");
            let rule_number: u8 = splits.next().unwrap().parse().unwrap();
            let rule = Rule::new(splits.next().unwrap(), rule_number, is_part2);

            (rule_number, rule)
        })
        .collect()
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut sections = INPUT.split("\n\n");
    let rule_section = sections.next().unwrap();
    let string_section = sections.next().unwrap();
    let rules = get_rules(rule_section, false);

    let part1 = string_section
        .lines()
        .map(|line| parse_rule_number(&rules, 0, line))
        .filter_map(Result::ok)
        .filter(|rest| rest.is_empty())
        .count();

    println!("part1 = {part1}");
}

fn part2() {
    let mut sections = INPUT.split("\n\n");
    let rule_section = sections.next().unwrap();
    let string_section = sections.next().unwrap();
    let rules = get_rules(rule_section, true);

    let part2 = string_section
        .lines()
        .map(|line| parse_rule_number(&rules, 0, line))
        .filter_map(Result::ok)
        .filter(|rest| rest.is_empty())
        .count();

    println!("part2 = {part2}");
}

fn parse_rule_number<'a>(
    rules: &'a HashMap<u8, Rule>,
    rule_number: u8,
    string: &'a str,
) -> Result<&'a str, (&'a Rule<'a>, &'a str)> {
    let rule = rules.get(&rule_number).unwrap();
    parse_rule(rules, rule, string)
}

fn parse_rule<'a>(
    rules: &'a HashMap<u8, Rule>,
    rule: &'a Rule,
    string: &'a str,
) -> Result<&'a str, (&'a Rule<'a>, &'a str)> {
    use Rule::*;

    match rule {
        Literal(literal) => {
            if string.starts_with(*literal) {
                let rest = &string[literal.len()..];
                Ok(rest)
            } else {
                Err((rule, string))
            }
        }
        Alias(alias) => parse_rule_number(rules, *alias, string),
        Sequence(sequence) => {
            let mut rest = string;

            for alias in sequence {
                match parse_rule_number(rules, *alias, rest) {
                    Ok(rest2) => rest = rest2,
                    Err(err) => return Err(err),
                };
            }

            Ok(rest)
        }
        Alt(parts) => {
            for inner_rule in parts {
                let result = parse_rule(rules, inner_rule, string);
                match result {
                    Ok(_) => return result,
                    Err(_) => continue,
                }
            }

            Err((rule, string))
        }
        Final(a, b) => {
            let mut left = string;
            let rest = parse_rule_number(rules, *a, left)?;
            left = rest;
            let mut a_count = 1;

            loop {
                let left_before_b = left;
                for _ in 0..(a_count - 1) {
                    match parse_rule_number(rules, *b, left) {
                        Ok(rest) => {
                            if rest.is_empty() {
                                return Ok(rest);
                            }
                            left = rest;
                        }
                        Err(_) => break,
                    }
                }

                let rest = parse_rule_number(rules, *a, left_before_b)?;
                left = rest;
                a_count += 1;
            }
        }
    }
}
