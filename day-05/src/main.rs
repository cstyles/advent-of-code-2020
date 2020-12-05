static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct Seat {
    row: i64,
    column: i64,
}

impl Seat {
    fn id(&self) -> i64 {
        self.row * 8 + self.column
    }

    fn next(&self) -> Self {
        if self.column == 7 {
            Self { row: self.row + 1, column: 0 }
        } else {
            Self { row: self.row, column: self.column + 1 }
        }
    }
}

impl std::convert::From<&str> for Seat {
    fn from(line: &str) -> Self {
        let row_str = line.get(0..7).expect("invalid row_str");
        let column_str = line.get(7..10).expect("invalid column_str");

        let mut multiplier = 64;
        let mut row = 0;
        for character in row_str.chars() {
            let bit = match character {
                'F' => 0,
                'B' => 1,
                _ => {
                    eprintln!("invalid row character");
                    panic!();
                },
            };

            row += bit * multiplier;
            multiplier >>= 1;
        }

        let mut multiplier = 4;
        let mut column = 0;
        for character in column_str.chars() {
            let bit = match character {
                'L' => 0,
                'R' => 1,
                _ => {
                    eprintln!("invalid row character");
                    panic!();
                },
            };

            column += bit * multiplier;
            multiplier >>= 1;
        }

        Seat { row, column }
    }
}

use std::cmp::{Ordering, PartialOrd, Ord};

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.row == other.row {
            self.column.cmp(&other.column)
        } else {
            self.row.cmp(&other.row)
        }
    }
}

fn main() {
    let max_id: i64 = INPUT
        .trim()
        .lines()
        .map(Seat::from)
        .map(|seat| seat.id())
        .max()
        .unwrap();

    println!("max_id: {}", max_id);

    // ====

    let mut seats: Vec<Seat> = INPUT
        .trim()
        .lines()
        .map(Seat::from)
        .collect();

    seats.sort();

    for seats in seats.windows(2) {
        if seats[0].next() == seats[1] {
            continue;
        } else {
            println!("my seat: {}", seats[0].next().id());
        }
    }
}
