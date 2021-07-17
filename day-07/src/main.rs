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
    part2();
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

    // println!("{:#?}", can_contain_shiny_gold);
    println!("{:#?}", can_contain_shiny_gold.len());
}

fn part2() {
    // Key: some bag type
    // Value: All bag types that are directly contained by the above bag type
    let mut hm: HashMap<&str, HashSet<(i32, &str)>> = Default::default();
    let re = Regex::new(r#"(?P<number>\d+ )?(?P<bagtype>\w+ \w+) bags?"#).unwrap();

    for line in INPUT.lines() {
        let mut key: &str = Default::default();

        for capture in re.captures_iter(line) {
            let bagtype = capture.name("bagtype").unwrap().as_str();
            let number = capture
                .name("number")
                .map(|n| n.as_str().trim().parse().unwrap());

            if capture.name("number").is_none() {
                key = bagtype;
            } else {
                let number = number.unwrap();
                hm.entry(key)
                    .and_modify(|set| {
                        set.insert((number, bagtype));
                    })
                    .or_insert_with(|| hashset!((number, bagtype)));
            }
        }
    }

    let dyn_prog: HashMap<&str, i32> = Default::default();
    let total = recursive_bags(&dyn_prog, &hm, "shiny gold");

    println!("total: {}", total);
}

fn recursive_bags(
    dyn_prog: &HashMap<&str, i32>,
    bag_data: &HashMap<&str, HashSet<(i32, &str)>>,
    bagtype: &str,
) -> i32 {
    if dyn_prog.contains_key(bagtype) {
        *dyn_prog.get(bagtype).unwrap()
    } else {
        let mut total = 0;

        match bag_data.get(bagtype) {
            Some(set) => {
                for directly_contained in set {
                    let (number, bagtype) = directly_contained;
                    total += number;
                    total += number * recursive_bags(&dyn_prog, &bag_data, bagtype);
                }
            }
            None => total = 0,
        }

        total
    }
}
