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

    fn multiply(&self, multiplier: i64) -> Self {
        Self {
            x: self.x * multiplier,
            y: self.y * multiplier,
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn rotate(&mut self, degrees: i64) -> Self {
        match degrees {
            90 => Point {
                x: self.y,
                y: -self.x,
            },
            180 => Point {
                x: -self.x,
                y: -self.y,
            },
            270 => Point {
                x: -self.y,
                y: self.x,
            },
            _ => panic!(),
        }
    }
}

fn main() {
    part1();
    part2();
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
            "L" => facing += value,
            "R" => facing += 360 - value,
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

fn part2() {
    let mut position = Point::default();
    let mut waypoint = Point { x: 10, y: 1 };

    for instruction in INPUT.lines() {
        let (action, value) = instruction.split_at(1);
        let value: i64 = value.parse().unwrap();

        match action {
            "N" => waypoint.y += value,
            "S" => waypoint.y -= value,
            "E" => waypoint.x += value,
            "W" => waypoint.x -= value,
            "R" => waypoint = waypoint.rotate(value),
            "L" => waypoint = waypoint.rotate(360 - value),
            "F" => {
                let vector = waypoint.multiply(value);
                position = position.add(&vector);
            }
            _ => panic!(),
        }
    }

    println!("part2: {}", position.manhattan_distance());
}
