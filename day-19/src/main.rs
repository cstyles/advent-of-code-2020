use std::collections::HashMap;
// use std::convert::From;

// static INPUT: &str = include_str!("../input-part1.txt");
// static INPUT: &str = include_str!("../input-part2.txt");
// static INPUT: &str = include_str!("../test-input.txt");
// static INPUT: &str = include_str!("../test-input2.txt");
static INPUT: &str = include_str!("../test-input3.txt");
// static INPUT: &str = include_str!("../my-input.txt");

#[derive(Debug)]
enum Rule<'a> {
    Literal(&'a str),       // 0: "a"
    Alias(&'a str),         // 0: 1
    Sequence(Vec<&'a str>), // 0: 1 2
    // RecursiveSequence(Vec<&'a str>),
    Alt(Vec<Rule<'a>>), // 0: 1 | 2
}

impl<'a> Rule<'a> {
    fn new(string: &'a str, rule_number: &str) -> Self {
        let string = string.trim();

        if string == "\"a\"" {
            Rule::Literal("a")
        } else if string == "\"b\"" {
            Rule::Literal("b")
        } else if string.contains('|') {
            let alt_parts = string
                .split('|')
                .map(|s| Rule::new(s.trim(), rule_number))
                .collect();

            Rule::Alt(alt_parts)
        } else if string.contains(' ') {
            let seq_parts: Vec<&str> = string.split(' ').collect();
            // let recursive = seq_parts.contains(&rule_number);
            // println!("rule_number: {}; recursive: {}", rule_number, recursive);

            Rule::Sequence(seq_parts)
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
            let rule = Rule::new(splits.next().unwrap(), rule_number);

            (rule_number, rule)
        })
        .collect();

    // println!("{:#?}", rules);

    let part1 = string_section
        .lines()
        .inspect(|line| println!("{}", line))
        .filter_map(|line| parse_rule_number(&rules, "0", line).ok())
        .filter(|(rest, _parsed)| rest.is_empty())
        .count();

    println!("part1 = {}", part1);
}

fn parse_rule_number<'a>(
    rules: &'a HashMap<&str, Rule>,
    rule_number: &str,
    string: &'a str,
) -> Result<(&'a str, &'a str), ()> {
    let rule = rules.get(rule_number).unwrap();
    parse_rule(rules, rule, string)
}

fn parse_rule<'a>(
    rules: &'a HashMap<&str, Rule>,
    rule: &'a Rule,
    string: &'a str,
) -> Result<(&'a str, &'a str), ()> {
    use Rule::*;

    match rule {
        Literal(literal) => {
            if string.starts_with(*literal) {
                let rest = &string[literal.len()..];
                let parsed = literal;

                Ok((rest, parsed))
            } else {
                Err(())
            }
        }
        Alias(alias) => parse_rule_number(rules, alias, string),
        Sequence(sequence) => {
            let mut rest = string;
            let mut parsed = String::new();

            for alias in sequence {
                let result = parse_rule_number(rules, alias, rest);

                match result {
                    Ok((rest2, parsed2)) => {
                        rest = rest2;
                        parsed.push_str(parsed2);
                    }
                    Err(err) => return Err(err),
                };
            }

            Ok((rest, "ugh"))
        }
        Alt(parts) => {
            for rule in parts {
                let result = parse_rule(rules, rule, string);
                // println!("Alt({:?}) => {:?}", rule, result);
                match result {
                    Ok(_) => return result,
                    Err(_) => continue,
                }
            }

            Err(())
        }
    }
}
