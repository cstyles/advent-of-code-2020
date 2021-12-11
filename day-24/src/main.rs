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
            Self::White => write!(f, "W"),
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

#[derive(Debug)]
struct HexGrid<T>(Vec<Vec<T>>);

impl<T> HexGrid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        Self(grid)
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

    pub fn trace(&self, mut y: usize, mut x: usize, path: Path) -> (usize, usize) {
        for direction in path.0 {
            let tile = self.move_(y, x, direction);
            y = tile.0;
            x = tile.1;
        }

        (y, x)
    }

    pub fn get_mut(&mut self, y: usize, x: usize) -> Option<&mut T> {
        self.0.get_mut(y).and_then(|row| row.get_mut(x))
    }
}

impl<T: Display> HexGrid<T> {
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

    let grid = vec![vec![Color::default(); 40]; 40];
    let mut grid = HexGrid::new(grid);

    let (y, x) = (20, 20);

    for path in paths {
        let (ty, tx) = grid.trace(y, x, path);
        grid.get_mut(ty, tx).unwrap().flip();
    }

    // grid.debug();

    let mut count = 0;
    for row in grid.0 {
        count += row.iter().filter(|&&tile| tile == Color::Black).count();
    }

    println!("part1 = {}", count);
}
