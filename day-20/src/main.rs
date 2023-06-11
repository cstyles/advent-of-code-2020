use std::collections::{HashMap, VecDeque};
use std::convert::TryInto;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pixel {
    Dot,
    Pound,
}

impl Pixel {
    fn as_char(&self) -> char {
        match self {
            Self::Dot => '.',
            Self::Pound => '#',
        }
    }
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

#[derive(Clone, Copy)]
struct Tile {
    id: usize,
    map: [[Pixel; 10]; 10],
}

impl std::hash::Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Tile {}

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

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::with_capacity(100);
        for row in self.map {
            for c in row.iter().map(Pixel::as_char) {
                string.push(c);
            }
            string.push('\n');
        }
        write!(f, "{string}")
    }
}

impl Tile {
    fn row(&self, y: usize) -> [Pixel; 10] {
        self.map[y]
    }

    fn column(&self, x: usize) -> [Pixel; 10] {
        let v: Vec<Pixel> = self.map.into_iter().map(|row| row[x]).collect();
        v.try_into().unwrap()
    }

    fn borders(&self) -> [Border; 4] {
        [self.top(), self.right(), self.bottom(), self.left()]
    }

    fn top(&self) -> Border {
        Border(self.row(0))
    }

    fn bottom(&self) -> Border {
        Border(self.row(self.map.len() - 1))
    }

    fn left(&self) -> Border {
        Border(self.column(0))
    }

    fn right(&self) -> Border {
        Border(self.column(self.map.len() - 1))
    }

    // TODO: I think order matters. Gonna say rotate, then flip.
    fn transform(&self, transform: Transform) -> Self {
        let Transform { rotate, flip } = transform;
        let rotated = self.rotate(rotate);
        rotated.flip(flip)
    }

    fn rotate(self, rotate: Rotate) -> Self {
        match rotate {
            Rotate::None => self,
            Rotate::Rotate90 => self.rotate90(),
            Rotate::Rotate180 => self.rotate180(),
            Rotate::Rotate270 => self.rotate270(),
        }
    }

    fn rotate90(self) -> Self {
        let mut map = [[Pixel::Dot; 10]; 10];

        for (i, row) in map.iter_mut().enumerate() {
            let mut col = self.column(i);
            col.reverse();
            *row = col;
        }

        Self { map, ..self }
    }

    fn rotate180(self) -> Self {
        let horz = self.flip_horizontal();
        horz.flip_vertical()
    }

    fn rotate270(self) -> Self {
        let mut map = [[Pixel::Dot; 10]; 10];

        for (i, row) in map.iter_mut().enumerate() {
            *row = self.column(9 - i);
        }

        Self { map, ..self }
    }

    fn flip(self, flip: Flip) -> Self {
        match flip {
            Flip::None => self,
            Flip::Horizontal => self.flip_horizontal(),
            Flip::Vertical => self.flip_vertical(),
        }
    }

    fn flip_horizontal(mut self) -> Self {
        for row in self.map.iter_mut() {
            row.reverse();
        }

        self
    }

    fn flip_vertical(self) -> Self {
        let map_vec: Vec<[Pixel; 10]> = self.map.into_iter().rev().collect();
        let map: [[Pixel; 10]; 10] = map_vec.try_into().unwrap();
        Self { map, ..self }
    }

    fn trim(self) -> TrimmedTile {
        let mut map = [[Pixel::Dot; 8]; 8];
        for (y, row) in self.map.into_iter().skip(1).take(8).enumerate() {
            for (x, pixel) in row.into_iter().skip(1).take(8).enumerate() {
                map[y][x] = pixel;
            }
        }

        TrimmedTile { map }
    }
}

#[derive(Clone, Copy)]
struct TrimmedTile {
    map: [[Pixel; 8]; 8],
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../test.txt");
    let tiles: Vec<Tile> = input.split("\n\n").map(Tile::from).collect();

    let corners = part1(tiles.clone());
    part2(tiles, corners);
}

fn part1(tiles: Vec<Tile>) -> Vec<usize> {
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

    let product: usize = corners.iter().product();
    println!("part1 = {}", product);
    corners
}

fn part2(tiles: Vec<Tile>, corners: Vec<usize>) {
    let first_corner = tiles
        .iter()
        .find(|tile| corners.contains(&tile.id))
        .unwrap();

    let mut places: HashMap<Tile, (isize, isize)> = HashMap::with_capacity(tiles.len());
    places.insert(*first_corner, (0, 0));
    let mut queue: VecDeque<Tile> = [*first_corner].into();

    loop {
        let Some(looking_at) = queue.pop_front() else { break };
        let &(y, x) = places.get(&looking_at).unwrap();

        for tile in tiles.iter() {
            if looking_at == *tile {
                continue;
            }

            if places.contains_key(tile) {
                continue;
            }

            let match_ = try_match(&looking_at, tile);
            match match_ {
                Match::None => {
                    continue;
                }
                Match::Top(transform) => {
                    let tile = tile.transform(transform);
                    queue.push_back(tile);
                    places.insert(tile, (y - 1, x));
                }
                Match::Right(transform) => {
                    let tile = tile.transform(transform);
                    queue.push_back(tile);
                    places.insert(tile, (y, x + 1));
                }
                Match::Bottom(transform) => {
                    let tile = tile.transform(transform);
                    queue.push_back(tile);
                    places.insert(tile, (y + 1, x));
                }
                Match::Left(transform) => {
                    let tile = tile.transform(transform);
                    queue.push_back(tile);
                    places.insert(tile, (y, x - 1));
                }
            }
        }
    }

    let shuffle: HashMap<(isize, isize), Tile> = places
        .into_iter()
        .map(|(tile, (y, x))| ((y, x), tile))
        .collect();

    let min_y = *shuffle.keys().map(|(y, _)| y).min().unwrap();
    let max_y = *shuffle.keys().map(|(y, _)| y).max().unwrap();
    let min_x = *shuffle.keys().map(|(_, x)| x).min().unwrap();
    let max_x = *shuffle.keys().map(|(_, x)| x).max().unwrap();

    let dimension = (max_y - min_y + 1) as usize;
    let tile_dimension = 10 - 2; // after trimming
    let size = dimension * tile_dimension;

    let mut final_map: Vec<Vec<Pixel>> = vec![Vec::with_capacity(size); size];

    for (y2, y) in (min_y..=max_y).enumerate() {
        for x in min_x..=max_x {
            let Some(tile) = shuffle.get(&(y, x)) else { continue };
            for (x2, row) in tile.trim().map.into_iter().enumerate() {
                final_map[y2 * tile_dimension + x2].extend_from_slice(&row);
            }
        }
    }

    let mut monsters;
    loop {
        monsters = count_monsters(&final_map);
        if monsters == 0 {
            final_map = rotate_map(final_map);
        } else {
            break;
        }
    }

    let total_pounds = total_pounds(&final_map);
    // assumes that no monsters overlap
    let sea_monster_pounds = 15 * monsters;
    println!("part2 = {}", total_pounds - sea_monster_pounds);
}

fn column(map: &[Vec<Pixel>], x: usize) -> Vec<Pixel> {
    map.iter().map(|row| row[x]).collect()
}

fn rotate_map(map: Vec<Vec<Pixel>>) -> Vec<Vec<Pixel>> {
    let mut new = vec![vec![Pixel::Dot; map.len()]; map.len()];

    for (i, row) in new.iter_mut().enumerate() {
        let mut col = column(&map, i);
        col.reverse();
        *row = col;
    }

    new
}

fn count_monsters(map: &[Vec<Pixel>]) -> usize {
    let mut count = 0;

    for three_rows in map.windows(3) {
        let (a, b, c) = (&three_rows[0], &three_rows[1], &three_rows[2]);
        let mut a_windows = a.windows(20);
        let mut b_windows = b.windows(20);
        let mut c_windows = c.windows(20);

        while let (Some(top), Some(mid), Some(bot)) =
            (a_windows.next(), b_windows.next(), c_windows.next())
        {
            if top[18] == Pixel::Pound
                && mid[0] == Pixel::Pound
                && mid[5] == Pixel::Pound
                && mid[6] == Pixel::Pound
                && mid[11] == Pixel::Pound
                && mid[12] == Pixel::Pound
                && mid[17] == Pixel::Pound
                && mid[18] == Pixel::Pound
                && mid[19] == Pixel::Pound
                && bot[1] == Pixel::Pound
                && bot[4] == Pixel::Pound
                && bot[7] == Pixel::Pound
                && bot[10] == Pixel::Pound
                && bot[13] == Pixel::Pound
                && bot[16] == Pixel::Pound
            {
                count += 1;
            }
        }
    }

    count
}

fn total_pounds(map: &[Vec<Pixel>]) -> usize {
    map.iter()
        .flatten()
        .filter(|pixel| **pixel == Pixel::Pound)
        .count()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Match {
    None,
    Top(Transform),
    Right(Transform),
    Bottom(Transform),
    Left(Transform),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Transform {
    rotate: Rotate,
    flip: Flip,
}

impl Transform {
    fn new(rotate: Rotate, flip: Flip) -> Self {
        Self { rotate, flip }
    }

    fn none() -> Self {
        Self {
            rotate: Rotate::None,
            flip: Flip::None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Rotate {
    None,
    Rotate90,
    Rotate180,
    Rotate270,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Flip {
    None,
    Horizontal,
    Vertical,
}

/// Returns a Match that indicates
/// 1) Which side of `a` matches up with `b`
/// 2) How `b` needs to be transformed to match up with `a`
fn try_match(a: &Tile, b: &Tile) -> Match {
    if a.top() == b.bottom() {
        Match::Top(Transform::none())
    } else if a.top() == b.bottom().reverse() {
        Match::Top(Transform::new(Rotate::None, Flip::Horizontal))
    } else if a.top() == b.top() {
        Match::Top(Transform::new(Rotate::None, Flip::Vertical))
    } else if a.top() == b.top().reverse() {
        Match::Top(Transform::new(Rotate::Rotate180, Flip::None))
    } else if a.top() == b.left() {
        Match::Top(Transform::new(Rotate::Rotate270, Flip::None))
    } else if a.top() == b.left().reverse() {
        Match::Top(Transform::new(Rotate::Rotate270, Flip::Horizontal)) // r90, vert
    } else if a.top() == b.right() {
        Match::Top(Transform::new(Rotate::Rotate90, Flip::Horizontal)) // r270, vert
    } else if a.top() == b.right().reverse() {
        Match::Top(Transform::new(Rotate::Rotate90, Flip::None))
    } else if a.bottom() == b.bottom() {
        Match::Bottom(Transform::new(Rotate::None, Flip::Vertical))
    } else if a.bottom() == b.bottom().reverse() {
        Match::Bottom(Transform::new(Rotate::Rotate180, Flip::None))
    } else if a.bottom() == b.top() {
        Match::Bottom(Transform::none())
    } else if a.bottom() == b.top().reverse() {
        Match::Bottom(Transform::new(Rotate::None, Flip::Horizontal))
    } else if a.bottom() == b.left() {
        Match::Bottom(Transform::new(Rotate::Rotate90, Flip::Horizontal)) // r270, vert
    } else if a.bottom() == b.left().reverse() {
        Match::Bottom(Transform::new(Rotate::Rotate90, Flip::None))
    } else if a.bottom() == b.right() {
        Match::Bottom(Transform::new(Rotate::Rotate270, Flip::None))
    } else if a.bottom() == b.right().reverse() {
        Match::Bottom(Transform::new(Rotate::Rotate270, Flip::Horizontal)) // r90, vert
    } else if a.left() == b.bottom() {
        Match::Left(Transform::new(Rotate::Rotate270, Flip::Vertical)) // r90, horz
    } else if a.left() == b.bottom().reverse() {
        Match::Left(Transform::new(Rotate::Rotate270, Flip::None))
    } else if a.left() == b.top() {
        Match::Left(Transform::new(Rotate::Rotate90, Flip::None))
    } else if a.left() == b.top().reverse() {
        Match::Left(Transform::new(Rotate::Rotate90, Flip::Vertical)) // r270, horz
    } else if a.left() == b.left() {
        Match::Left(Transform::new(Rotate::None, Flip::Horizontal))
    } else if a.left() == b.left().reverse() {
        Match::Left(Transform::new(Rotate::Rotate180, Flip::None))
    } else if a.left() == b.right() {
        Match::Left(Transform::none())
    } else if a.left() == b.right().reverse() {
        Match::Left(Transform::new(Rotate::None, Flip::Vertical))
    } else if a.right() == b.bottom() {
        Match::Right(Transform::new(Rotate::Rotate90, Flip::None))
    } else if a.right() == b.bottom().reverse() {
        Match::Right(Transform::new(Rotate::Rotate90, Flip::Vertical)) // r270, horz
    } else if a.right() == b.top() {
        Match::Right(Transform::new(Rotate::Rotate270, Flip::Vertical)) // r90, horz
    } else if a.right() == b.top().reverse() {
        Match::Right(Transform::new(Rotate::Rotate270, Flip::None))
    } else if a.right() == b.left() {
        Match::Right(Transform::none())
    } else if a.right() == b.left().reverse() {
        Match::Right(Transform::new(Rotate::None, Flip::Vertical))
    } else if a.right() == b.right() {
        Match::Right(Transform::new(Rotate::None, Flip::Horizontal))
    } else if a.right() == b.right().reverse() {
        Match::Right(Transform::new(Rotate::Rotate180, Flip::None))
    } else {
        Match::None
    }
}
