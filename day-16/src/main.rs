use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::ops::RangeInclusive;

static INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Field<'a> {
    name: &'a str,
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}

impl<'a> From<&'a str> for Field<'a> {
    fn from(string: &'a str) -> Self {
        let colon_splitter = ": ";
        let colon_split_index = string.find(colon_splitter).unwrap();
        let name = &string[0..colon_split_index];
        let string = &string[colon_split_index + colon_splitter.len()..];

        let or_splitter = " or ";
        let or_split_index = string.find(or_splitter).unwrap();
        let range1 = &string[0..or_split_index];
        let range2 = &string[or_split_index + or_splitter.len()..];

        let range1 = range_str_to_range(range1);
        let range2 = range_str_to_range(range2);

        Self {
            name,
            range1,
            range2,
        }
    }
}

fn range_str_to_range(string: &str) -> RangeInclusive<usize> {
    let mut iter = string.split('-').map(|r| r.parse().unwrap());
    let lower = iter.next().unwrap();
    let upper = iter.next().unwrap();
    lower..=upper
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = INPUT.lines();

    let ranges: Vec<RangeInclusive<usize>> =
        lines
            .clone()
            .take(20)
            .map(|line| Field::from(line))
            .fold(vec![], |mut ranges, field| {
                ranges.push(field.range1);
                ranges.push(field.range2);
                ranges
            });

    let lines = lines.skip(5);
    let sum: usize = lines
        .flat_map(|line| line.split(','))
        .filter_map(|num| num.parse().ok())
        .fold(0, |sum, num: usize| {
            for range in ranges.iter() {
                if range.contains(&num) {
                    return sum;
                }
            }

            sum + num
        });

    println!("part1 = {}", sum);
}

fn part2() {
    let fields: Vec<Field> = INPUT
        .lines()
        .take(20)
        .map(|line| Field::from(line))
        .collect();

    let field_name_set: HashSet<&str> = fields.iter().map(|field| field.name).collect();

    let mut possible_field_names: Vec<HashSet<&str>> = Vec::with_capacity(20);
    for _ in 0..20 {
        possible_field_names.push(field_name_set.clone());
    }

    let ranges: Vec<RangeInclusive<usize>> = fields.iter().fold(vec![], |mut ranges, field| {
        ranges.push(field.range1.clone());
        ranges.push(field.range2.clone());
        ranges
    });

    let lines: Vec<&str> = INPUT
        .lines()
        .skip(25)
        .filter(|line| {
            for num in line.split(',').filter_map(|num| num.parse::<usize>().ok()) {
                let mut valid_number = false;
                for range in ranges.iter() {
                    if range.contains(&num) {
                        valid_number = true;
                        break;
                    }
                }

                if !valid_number {
                    return false;
                }
            }

            true
        })
        .collect();

    let numbers: Vec<usize> = lines
        .iter()
        .flat_map(|line| line.split(','))
        .filter_map(|num| num.parse().ok())
        .collect();

    for (i, number) in numbers.iter().enumerate() {
        let position = i % 20;
        let possible_fields = possible_field_names.get_mut(position).unwrap();

        for field in fields.iter() {
            if !field.range1.contains(&number) && !field.range2.contains(&number) {
                possible_fields.remove(field.name);
            }
        }
    }

    // Use definite knowledge to whittle down possible field names
    let mut done = HashMap::<&str, usize>::with_capacity(20);
    while done.len() < 20 {
        for (position, fields) in possible_field_names.iter_mut().enumerate() {
            if fields.len() == 1 {
                done.insert(fields.iter().next().unwrap(), position);
            } else {
                for (name, _position) in done.iter() {
                    fields.remove(*name);
                }
            }
        }
    }

    let my_ticket: Vec<usize> = INPUT
        .lines()
        .skip(22)
        .next()
        .unwrap()
        .split(',')
        .filter_map(|num| num.parse().ok())
        .collect();

    let positions: Vec<usize> = done.iter()
        .filter(|(field_name, _position)| field_name.starts_with("departure"))
        .map(|(_field_name, position)| *position)
        .collect();

    let mut product = 1;
    for position in positions {
        product *= my_ticket[position];
    }

    println!("part2 = {}", product);
}
