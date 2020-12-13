// static INPUT: &str = include_str!("../test-input.txt");
static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Default)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    part1();
}

fn part1() {
    let mut facing: i64 = 0;
    let mut position = Point::default();

    for instruction in INPUT.lines() {
        let (action, value) = instruction.split_at(1);
        let value: i64 = value.parse().unwrap();

        match action {
            "N" => position.y += value,
            "S" => position.y -= value,
            "E" => position.x += value,
            "W" => position.x -= value,
            "L" => facing = facing + value,
            "R" => facing = facing + 360 - value,
            "F" => {
                match facing % 360 {
                    0 => position.x += value,   // West
                    90 => position.y += value,  // North
                    180 => position.x -= value, // East
                    270 => position.y -= value, // South
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    println!("part1: {}", position.manhattan_distance());
}
