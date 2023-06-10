use std::convert::TryInto;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pixel {
    Dot,
    Pound,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Pixel::Dot => write!(f, "."),
            Pixel::Pound => write!(f, "#"),
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel::Dot
    }
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '.' => Pixel::Dot,
            '#' => Pixel::Pound,
            _ => unreachable!("bad pixel: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Border([Pixel; 10]);

impl Display for Border {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = self.0.into_iter().map(|pixel| pixel.to_string()).collect();

        write!(f, "{}", string)
    }
}

impl Border {
    fn reverse(&self) -> Border {
        let mut array = self.0;
        array.reverse();
        Border(array)
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    id: usize,
    map: [[Pixel; 10]; 10],
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl From<&str> for Tile {
    fn from(string: &str) -> Self {
        let lines: Vec<&str> = string.lines().collect();
        let id = lines[0][5..9].parse().unwrap();

        let mut map: [[Pixel; 10]; 10] = Default::default();
        for (row, line) in lines[1..].iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                map[row][i] = Pixel::from(c);
            }
        }

        Self { id, map }
    }
}

impl Tile {
    fn borders(&self) -> [Border; 4] {
        [self.top(), self.right(), self.bottom(), self.left()]
    }

    fn top(&self) -> Border {
        Border(self.map[0])
    }

    fn bottom(&self) -> Border {
        Border(self.map[self.map.len() - 1])
    }

    fn left(&self) -> Border {
        Border(
            self.map
                .into_iter()
                .map(|row| row[0])
                .collect::<Vec<Pixel>>()
                .try_into()
                .unwrap(),
        )
    }

    fn right(&self) -> Border {
        Border(
            self.map
                .into_iter()
                .map(|row| row[row.len() - 1])
                .collect::<Vec<Pixel>>()
                .try_into()
                .unwrap(),
        )
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let tiles: Vec<Tile> = input.split("\n\n").map(Tile::from).collect();

    part1(tiles.clone());
    part2(tiles);
}

fn part1(tiles: Vec<Tile>) {
    let mut corners: Vec<usize> = vec![];
    for reference_tile in tiles.iter() {
        let mut matches = 0;
        for reference_border in reference_tile.borders() {
            for tile in tiles.iter() {
                if reference_tile == tile {
                    continue;
                }

                for border in tile.borders() {
                    if reference_border == border || reference_border == border.reverse() {
                        matches += 1;
                    }
                }
            }
        }

        if matches <= 2 {
            corners.push(reference_tile.id);
        }
    }

    let product: usize = corners.into_iter().product();
    println!("part1 = {}", product);
}

fn part2(tiles: Vec<Tile>) {
    // find a corner
    // place it in top left corner
    // find the two matching
    for reference_tile in tiles.iter() {
        for reference_border in reference_tile.borders() {
            for tile in tiles.iter() {
                if reference_tile == tile {
                    continue;
                }

                for border in tile.borders() {
                    if reference_border == border || reference_border == border.reverse() {
                        // whaaa??
                    }
                }
            }
        }
    }
}
