static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Default, Copy, Clone)]
struct Seat {
    row: i64,
    column: i64,
}

impl Seat {
    fn id(&self) -> i64 {
        self.row * 8 + self.column
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

fn main() {
    let max_id: i64 = INPUT
        .trim()
        .lines()
        .map(Seat::from)
        .map(|seat| seat.id())
        .max()
        .unwrap();

    println!("max_id: {}", max_id);
}
