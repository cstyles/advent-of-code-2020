// use std::convert::From;
// use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../input.txt");
// static INPUT: &str = include_str!("../test-input.txt");

fn main() {
    part1();
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    // TODO: return an iterator?
    // fn neighbors(&self) -> Vec<Self> {
    fn neighbors(&self) -> HashSet<Self> {
        // let mut neighbors = Vec::with_capacity(26); // 9 + 8 + 9
        let mut neighbors = HashSet::default(); // 9 + 8 + 9

        for z in self.z - 1..=self.z + 1 {
            for y in self.y - 1..=self.y + 1 {
                for x in self.x - 1..=self.x + 1 {
                    let p = Point { x, y, z };
                    neighbors.insert(p);
                }
            }
        }

        neighbors.remove(self);

        neighbors
    }

    fn active_neigbors(&self, set: &HashSet<Point>) -> usize {
        self.neighbors()
            .iter()
            .filter(|neighbor| set.contains(neighbor))
            .count()
    }
}

fn part1() {
    let mut map: HashMap<Point, bool> = Default::default();

    INPUT.lines().enumerate().for_each(|(y, line)| {
        for (x, c) in line.chars().enumerate() {
            let y = y as isize;
            let x = x as isize;
            let point = Point { z: 0, y, x };
            let active = c == '#';

            map.insert(point, active);
        }
    });

    // debug(&map);

    for _ in 0..6 {
        let mut new_map = map.clone();

        let mut to_process: HashSet<Point> = HashSet::default();

        for (point, _active) in map.iter() {
            for neighbor in point.neighbors() {
                to_process.insert(neighbor);
            }
        }

        let active_set: HashSet<Point> = map
            .iter()
            .filter(|&(_point, active)| *active)
            .map(|(point, _active)| *point)
            .collect();

        for point in to_process {
            let active = map.get(&point).unwrap_or(&false);
            let active_neigbors = point.active_neigbors(&active_set);
            let new_active = match (active, active_neigbors) {
                (false, 3) => true,
                (false, _) => false,
                (true, 2..=3) => true,
                (true, _) => false,
            };

            new_map.insert(point, new_active);
        }

        map = new_map.clone();

        // debug(&map);
        // println!(
        //     "active cubes = {}",
        //     map.iter().filter(|&(_point, active)| *active).count()
        // );
        // println!();
        // println!("----====----");
        // println!();
    }

    println!(
        "part1 = {}",
        map.iter().filter(|&(_point, active)| *active).count()
    );

    // println!("  --  ==  --");
    // println!();
    // debug(&new_map);

    // for (Point { x, y, z }, active) in map.iter() {
    //     println!("({}, {}, {}) => {}", z, y, x, active);
    // }
    // let new_set =
}

fn debug(map: &HashMap<Point, bool>) {
    // TODO: calculate max z, y, x

    let zs = map.iter().map(|(point, _active)| point.z);
    let min_z = zs.clone().min().unwrap();
    let max_z = zs.max().unwrap();

    let ys = map.iter().map(|(point, _active)| point.y);
    let min_y = ys.clone().min().unwrap();
    let max_y = ys.max().unwrap();

    let xs = map.iter().map(|(point, _active)| point.x);
    let min_x = xs.clone().min().unwrap();
    let max_x = xs.max().unwrap();

    for z in min_z..=max_z {
        println!("z={}", z);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Point { x, y, z };

                let active = map.get(&p).unwrap_or(&false);
                let c = match active {
                    false => '.',
                    true => '#',
                };

                print!("{}", c);
            }

            println!();
        }

        println!();
    }

    println!();
}
