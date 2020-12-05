use std::cmp::{Ord, Ordering, PartialOrd};

static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Default, PartialEq, Eq)]
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
            Self {
                row: self.row + 1,
                column: 0,
            }
        } else {
            Self {
                row: self.row,
                column: self.column + 1,
            }
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
                }
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
                    eprintln!("invalid column character");
                    panic!();
                }
            };

            column += bit * multiplier;
            multiplier >>= 1;
        }

        Seat { row, column }
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.row.cmp(&other.row) {
            Ordering::Equal => self.column.cmp(&other.column),
            x => x,
        }
    }
}

fn main() {
    let mut seats: Vec<Seat> = INPUT.lines().map(Seat::from).collect();
    part1(&seats);

    seats.sort();
    part2(&seats);
}

fn part1(seats: &[Seat]) {
    let max_id = seats.iter().map(|seat| seat.id()).max().unwrap();

    println!("part1: {}", max_id);
}

fn part2(seats: &[Seat]) {
    for seats in seats.windows(2) {
        if seats[0].next() != seats[1] {
            println!("my seat: {}", seats[0].next().id());
            break;
        }
    }
}
