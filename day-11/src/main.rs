use std::convert::From;
use std::fmt;

static INPUT: &str = include_str!("../input.txt");
static NUMBER_OF_COLUMNS: usize = 94;
static NUMBER_OF_ROWS: usize = 97;

// static INPUT: &str = include_str!("../test-input.txt");
// static NUMBER_OF_COLUMNS: usize = 10;
// static NUMBER_OF_ROWS: usize = 10;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Tile {
    fn from(character: char) -> Self {
        use Tile::*;

        match character {
            '.' => Floor,
            'L' => Empty,
            '#' => Occupied,
            _ => unreachable!(),
        }
    }
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> Self {
        use Tile::*;

        match tile {
            Floor => '.',
            Empty => 'L',
            Occupied => '#',
        }
    }
}

impl From<&Tile> for u8 {
    fn from(tile: &Tile) -> Self {
        use Tile::*;

        match tile {
            Occupied => 1,
            _ => 0,
        }
    }
}

impl Tile {
    fn next(&self, occupied_neighbors: u8) -> Self {
        use Tile::*;

        match (self, occupied_neighbors) {
            (Floor, _) => Floor,
            (Empty, 0) => Occupied,
            (Empty, _) => Empty,
            (Occupied, 4..=8) => Empty,
            (Occupied, _) => Occupied,
        }
    }
}

#[derive(Default, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<Tile>>,
}

impl Map {
    fn next(&self) -> Self {
        let mut new = Map::default();
        for y in 0..NUMBER_OF_ROWS {
            new.map.push(vec![]);

            for x in 0..NUMBER_OF_COLUMNS {
                let occupied_neighbors = occupied_neighbors(&self.map, y, x);
                new.map[y].push(self.map[y][x].next(occupied_neighbors));
            }
        }
        new
    }

    fn occupied_seats(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|&tile| *tile == Tile::Occupied)
            .count()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(NUMBER_OF_COLUMNS * NUMBER_OF_ROWS);
        for row in self.map.iter() {
            for tile in row {
                s.push(tile.into());
            }

            s.push('\n');
        }
        // TODO: use join to get rid of final newline

        write!(f, "{}", s)
    }
}

fn main() {
    part1();
}

fn part1() {
    let map: Vec<Vec<Tile>> = INPUT
        .lines()
        .map(str::chars)
        .map(|chars| chars.map(Tile::from))
        .map(Iterator::collect)
        .collect();

    let mut map = Map { map };
    let mut next_map = map.next();

    while map != next_map {
        map.map = next_map.map;
        next_map = map.next();
    }

    println!("part1 = {}", map.occupied_seats());
}

fn occupied_neighbors(map: &[Vec<Tile>], y: usize, x: usize) -> u8 {
    if y == 0 {
        if x == 0 {
            u8::from(&map[y][x + 1]) + u8::from(&map[y + 1][x]) + u8::from(&map[y + 1][x + 1])
        } else if x == NUMBER_OF_COLUMNS - 1 {
            u8::from(&map[y][x - 1]) + u8::from(&map[y + 1][x - 1]) + u8::from(&map[y + 1][x])
        } else {
            u8::from(&map[y][x - 1])
                + u8::from(&map[y][x + 1])
                + u8::from(&map[y + 1][x - 1])
                + u8::from(&map[y + 1][x])
                + u8::from(&map[y + 1][x + 1])
        }
    } else if y == NUMBER_OF_ROWS - 1 {
        if x == 0 {
            u8::from(&map[y][x + 1]) + u8::from(&map[y - 1][x]) + u8::from(&map[y - 1][x + 1])
        } else if x == NUMBER_OF_COLUMNS - 1 {
            u8::from(&map[y][x - 1]) + u8::from(&map[y - 1][x - 1]) + u8::from(&map[y - 1][x])
        } else {
            u8::from(&map[y][x - 1])
                + u8::from(&map[y][x + 1])
                + u8::from(&map[y - 1][x - 1])
                + u8::from(&map[y - 1][x])
                + u8::from(&map[y - 1][x + 1])
        }
    } else {
        if x == 0 {
            u8::from(&map[y - 1][x])
                + u8::from(&map[y - 1][x + 1])
                + u8::from(&map[y][x + 1])
                + u8::from(&map[y + 1][x])
                + u8::from(&map[y + 1][x + 1])
        } else if x == NUMBER_OF_COLUMNS - 1 {
            u8::from(&map[y - 1][x - 1])
                + u8::from(&map[y - 1][x])
                + u8::from(&map[y][x - 1])
                + u8::from(&map[y + 1][x])
                + u8::from(&map[y + 1][x - 1])
        } else {
            u8::from(&map[y - 1][x - 1])
                + u8::from(&map[y - 1][x])
                + u8::from(&map[y - 1][x + 1])
                + u8::from(&map[y][x - 1])
                + u8::from(&map[y][x + 1])
                + u8::from(&map[y + 1][x - 1])
                + u8::from(&map[y + 1][x])
                + u8::from(&map[y + 1][x + 1])
        }
    }
}
