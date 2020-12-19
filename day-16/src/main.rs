use std::convert::From;
use std::ops::Range;

static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Default)]
struct Field<'a> {
    name: &'a str,
    range1: Range<usize>,
    range2: Range<usize>,
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

fn range_str_to_range(string: &str) -> Range<usize> {
    let mut iter = string.split('-').map(|r| r.parse().unwrap());
    iter.next().unwrap()..iter.next().unwrap()
}

fn main() {
    part1();
}

fn part1() {
    let lines = INPUT.lines();

    let ranges: Vec<Range<usize>> =
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
