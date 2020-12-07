#[macro_use]
extern crate maplit;
use maplit::hashset;

use regex::Regex;
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../input.txt");

#[derive(Default, PartialEq, Eq, Hash)]
struct BagType<'a> {
    // number: i32,
    modifier: &'a str,
    color: &'a str,
}

impl<'a> std::convert::From<(&'a str, &'a str)> for BagType<'a> {
    fn from(pair: (&'a str, &'a str)) -> Self {
        Self {
            modifier: pair.0,
            color: pair.1,
        }
    }
}

fn main() {
    part1();
}

fn part1() {
    // Key: some bag type
    // Value: All bag types that can (directly) contain the above bag type
    let mut hm: HashMap<&str, HashSet<&str>> = Default::default();
    let re = Regex::new(r#"(?P<number>\d+ )?(?P<bagtype>\w+ \w+) bags?"#).unwrap();

    for line in INPUT.lines() {
        let mut key: &str = Default::default();

        for capture in re.captures_iter(line) {
            let bagtype = capture.name("bagtype").unwrap().as_str();

            if capture.name("number").is_none() {
                key = bagtype;
            } else {
                hm.entry(bagtype)
                    .and_modify(|set| {
                        set.insert(key);
                    })
                    .or_insert_with(|| hashset!(key));
            }
        }
    }

    let directly_contains = hm.get("shiny gold").unwrap();
    let mut can_contain_shiny_gold: HashSet<&str> = hashset!();
    let mut to_check: Vec<&str> = directly_contains.iter().copied().collect();

    while !to_check.is_empty() {
        let bagtype = to_check.pop().unwrap();
        can_contain_shiny_gold.insert(bagtype);

        let can_contain_indirectly = match hm.get(bagtype) {
            Some(set) => set,
            None => continue,
        };

        let difference = can_contain_indirectly.difference(&can_contain_shiny_gold);

        let mut difference: Vec<&str> = difference.copied().collect();
        to_check.append(&mut difference);
    }

    println!("{:#?}", can_contain_shiny_gold);
    println!("{:#?}", can_contain_shiny_gold.len());
}
