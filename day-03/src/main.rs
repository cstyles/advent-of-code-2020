#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Tree,
    Open,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Open
    }
}

impl From<char> for Tile {
    fn from(character: char) -> Self {
        match character {
            '#' => Self::Tree,
            '.' => Self::Open,
            _ => Self::Open,
        }
    }
}

#[derive(Debug)]
struct Map {
    map: [[Tile; WIDTH]; HEIGHT],
}

impl Map {
    fn new(input: String) -> Self {
        let mut map = Self {
            map: [[Default::default(); WIDTH]; HEIGHT],
        };

        for (y, line) in input.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                map.set(y, x, tile.into());
            }
        }

        map
    }

    fn get(&self, y: usize, x: usize) -> Tile {
        self.map[y][x % WIDTH]
    }

    fn set(&mut self, y: usize, x: usize, tile: Tile) {
        self.map[y][x] = tile;
    }
}

const WIDTH: usize = 31;
const HEIGHT: usize = 323;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = Map::new(input);

    part1(&map);
    println!();
    part2(&map);
}

fn part1(map: &Map) {
    println!("== PART 1 ==");

    let y_slope: usize = 1;
    let x_slope: usize = 3;
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut trees = 0;

    while y < HEIGHT {
        if map.get(y, x) == Tile::Tree {
            trees += 1;
        }

        y += y_slope;
        x += x_slope;
    }

    println!("trees = {}", trees);
}

fn part2(map: &Map) {
    println!("== PART 2 ==");

    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut all_trees: Vec<i64> = vec![];

    for (y_slope, x_slope) in slopes.iter() {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut trees = 0;

        while y < HEIGHT {
            if map.get(y, x) == Tile::Tree {
                trees += 1;
            }

            y += y_slope;
            x += x_slope;
        }

        all_trees.push(trees);
    }

    let product: i64 = all_trees.iter().product();
    println!("trees = {:?}", product);
}
