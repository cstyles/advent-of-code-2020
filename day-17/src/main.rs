use std::collections::{HashMap, HashSet};
use std::hash::Hash;

static INPUT: &str = include_str!("../input.txt");
// static INPUT: &str = include_str!("../test-input.txt");

fn main() {
    generic_part::<Point3>(1);
    generic_part::<Point4>(2);
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Copy, Clone)]
struct Point3 {
    x: isize,
    y: isize,
    z: isize,
}

trait Point: Sized + Eq + Hash + Copy + Clone {
    fn new(y: isize, x: isize) -> Self;
    fn neighbors(&self) -> HashSet<Self>;

    fn active_neigbors(&self, set: &HashSet<Self>) -> usize {
        self.neighbors()
            .iter()
            .filter(|neighbor| set.contains(neighbor))
            .count()
    }
}

impl Point for Point3 {
    fn new(y: isize, x: isize) -> Self {
        Self { x, y, z: 0 }
    }

    fn neighbors(&self) -> HashSet<Self> {
        let mut neighbors = HashSet::with_capacity(26); // 9 + 8 + 9

        for z in self.z - 1..=self.z + 1 {
            for y in self.y - 1..=self.y + 1 {
                for x in self.x - 1..=self.x + 1 {
                    let p = Point3 { x, y, z };
                    neighbors.insert(p);
                }
            }
        }

        neighbors.remove(self);

        neighbors
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Copy, Clone)]
struct Point4 {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Point for Point4 {
    fn new(y: isize, x: isize) -> Self {
        Self { x, y, z: 0, w: 0 }
    }

    fn neighbors(&self) -> HashSet<Self> {
        let mut neighbors = HashSet::with_capacity(80); // 27 + 27 + 26

        for w in self.w - 1..=self.w + 1 {
            for z in self.z - 1..=self.z + 1 {
                for y in self.y - 1..=self.y + 1 {
                    for x in self.x - 1..=self.x + 1 {
                        let p = Point4 { x, y, z, w };
                        neighbors.insert(p);
                    }
                }
            }
        }

        neighbors.remove(self);

        neighbors
    }
}

fn generic_part<T: Point>(part_number: usize) {
    let mut map: HashMap<T, bool> = Default::default();

    INPUT.lines().enumerate().for_each(|(y, line)| {
        for (x, c) in line.chars().enumerate() {
            let y = y as isize;
            let x = x as isize;
            let point = T::new(y, x);
            let active = c == '#';

            map.insert(point, active);
        }
    });

    for _ in 0..6 {
        let mut new_map = map.clone();

        let mut to_process: HashSet<T> = HashSet::default();

        for (point, _active) in map.iter() {
            for neighbor in point.neighbors() {
                to_process.insert(neighbor);
            }
        }

        let active_set: HashSet<T> = map
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
    }

    println!(
        "part{} = {}",
        part_number,
        map.iter().filter(|&(_point, active)| *active).count()
    );
}
