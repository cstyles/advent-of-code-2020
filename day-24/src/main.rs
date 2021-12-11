use std::fmt::{Debug, Display};

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    #[allow(unused)]
    pub const fn all() -> [Self; 6] {
        [
            Self::East,
            Self::SouthEast,
            Self::SouthWest,
            Self::West,
            Self::NorthWest,
            Self::NorthEast,
        ]
    }
}

#[derive(Debug)]
struct Path(Vec<Direction>);

impl From<&str> for Path {
    fn from(string: &str) -> Self {
        use Direction::*;

        let mut path = vec![];
        let chars: Vec<_> = string.chars().collect();
        let mut chars = chars.as_slice();

        while let Some((c, tail)) = chars.split_first() {
            chars = tail;

            let direction = match c {
                'e' => East,
                'w' => West,
                'n' => {
                    let (c, tail) = chars.split_first().unwrap();
                    chars = tail;
                    match c {
                        'e' => NorthEast,
                        'w' => NorthWest,
                        _ => unreachable!("'n{}' is not a direction", c),
                    }
                }
                's' => {
                    let (c, tail) = chars.split_first().unwrap();
                    chars = tail;
                    match c {
                        'e' => SouthEast,
                        'w' => SouthWest,
                        _ => unreachable!("'s{}' is not a direction", c),
                    }
                }
                _ => unreachable!("invalid char: {}", c),
            };

            path.push(direction);
        }

        Self(path)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Color {
    Black,
    White,
}

impl Default for Color {
    fn default() -> Self {
        Self::White
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Black => write!(f, "B"),
            Self::White => write!(f, "_"),
        }
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Color {
    fn flip(&mut self) {
        *self = match *self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Debug, Clone)]
struct HexGrid<T>(Vec<Vec<T>>);

impl<T> HexGrid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        Self(grid)
    }

    #[allow(unused)]
    pub fn height(&self) -> usize {
        self.0.len()
    }

    #[allow(unused)]
    pub fn width(&self) -> usize {
        self.0.first().unwrap().len()
    }

    pub fn move_(&self, y: usize, x: usize, direction: Direction) -> (usize, usize) {
        use Direction::*;

        match direction {
            East => (y, x + 1),
            SouthEast => {
                if y % 2 == 0 {
                    (y + 1, x)
                } else {
                    (y + 1, x + 1)
                }
            }
            SouthWest => {
                if y % 2 == 0 {
                    (y + 1, x - 1)
                } else {
                    (y + 1, x)
                }
            }
            West => (y, x - 1),
            NorthWest => {
                if y % 2 == 0 {
                    (y - 1, x - 1)
                } else {
                    (y - 1, x)
                }
            }
            NorthEast => {
                if y % 2 == 0 {
                    (y - 1, x)
                } else {
                    (y - 1, x + 1)
                }
            }
        }
    }

    pub fn neighbors<const N: usize>(&self, y: usize, x: usize) -> [Option<(usize, usize)>; 6] {
        [
            wrapping_next::<N>(x).map(|x| (y, x)), // East
            if y % 2 == 0 {
                wrapping_next::<N>(y).map(|y| (y, x))
            } else {
                wrapping_next::<N>(y).zip(wrapping_next::<N>(x))
            }, // SouthEast
            if y % 2 == 0 {
                wrapping_next::<N>(y).zip(wrapping_prev::<N>(x))
            } else {
                wrapping_next::<N>(y).map(|y| (y, x))
            }, // SouthWest
            wrapping_prev::<N>(x).map(|x| (y, x)), // West
            if y % 2 == 0 {
                wrapping_prev::<N>(y).zip(wrapping_prev::<N>(x))
            } else {
                wrapping_prev::<N>(y).map(|y| (y, x))
            }, // NorthWest
            if y % 2 == 0 {
                wrapping_prev::<N>(y).map(|y| (y, x))
            } else {
                wrapping_prev::<N>(y).zip(wrapping_next::<N>(x))
            }, // NorthEast
        ]
    }

    pub fn neighbor_values<const N: usize>(&self, y: usize, x: usize) -> impl Iterator<Item = &T> {
        self.neighbors::<N>(y, x)
            .into_iter()
            .flatten()
            .map(|(y, x)| self.get_unchecked(y, x))
    }

    pub fn trace(&self, mut y: usize, mut x: usize, path: Path) -> (usize, usize) {
        for direction in path.0 {
            let tile = self.move_(y, x, direction);
            y = tile.0;
            x = tile.1;
        }

        (y, x)
    }

    #[allow(unused)]
    pub fn get(&self, y: usize, x: usize) -> Option<&T> {
        self.0.get(y).and_then(|row| row.get(x))
    }

    pub fn get_unchecked(&self, y: usize, x: usize) -> &T {
        unsafe { self.0.get_unchecked(y).get_unchecked(x) }
    }

    pub fn get_mut(&mut self, y: usize, x: usize) -> Option<&mut T> {
        self.0.get_mut(y).and_then(|row| row.get_mut(x))
    }
}

impl HexGrid<Color> {
    pub fn count_black(&self) -> usize {
        let mut count = 0;
        for row in &self.0 {
            count += row.iter().filter(|&&tile| tile == Color::Black).count();
        }

        count
    }

    pub fn evolve(&mut self) {
        let mut new_grid = self.clone();

        for (y, row) in self.0.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let adjacent_black_tiles = self
                    .neighbor_values::<200>(y, x)
                    .filter(|&&color| color == Color::Black)
                    .count();

                match tile {
                    Color::Black => {
                        if adjacent_black_tiles == 0 || adjacent_black_tiles > 2 {
                            new_grid.0[y][x] = Color::White;
                        }
                    }
                    Color::White => {
                        if adjacent_black_tiles == 2 {
                            new_grid.0[y][x] = Color::Black;
                        }
                    }
                }
            }
        }

        *self = new_grid;
    }
}

impl<T: Display> HexGrid<T> {
    #[allow(unused)]
    pub fn debug(&self) {
        for (y, row) in self.0.iter().enumerate() {
            if y % 2 != 0 {
                print!(" ");
            }

            // TODO: join?
            for cell in row.iter() {
                print!("{} ", cell);
            }

            println!();
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let paths: Vec<Path> = input.lines().map(Path::from).collect();

    let grid = vec![vec![Color::default(); 200]; 200];
    let mut grid = HexGrid::new(grid);

    let (y, x) = (100, 100);

    for path in paths {
        let (ty, tx) = grid.trace(y, x, path);
        grid.get_mut(ty, tx).unwrap().flip();
    }

    // grid.debug();

    println!("part1 = {}", grid.count_black());
    println!();

    for day in 1..=100 {
        grid.evolve();
        if day <= 10 || day % 10 == 0 {
            println!("day {} = {}", day, grid.count_black());
            // grid.debug();
        }
    }
}

const fn wrapping_prev<const N: usize>(lhs: usize) -> Option<usize> {
    if lhs == 0 {
        None
    } else {
        Some(lhs - 1)
    }
}

const fn wrapping_next<const N: usize>(lhs: usize) -> Option<usize> {
    if lhs >= N - 1 {
        None
    } else {
        Some(lhs + 1)
    }
}
