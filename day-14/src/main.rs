use std::convert::From;

static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Copy, Clone)]
enum Bit {
    Zero,
    One,
    Copy,
}

impl Default for Bit {
    fn default() -> Self {
        Self::Zero
    }
}

impl From<char> for Bit {
    fn from(c: char) -> Self {
        use Bit::*;

        match c {
            '0' => Zero,
            '1' => One,
            'X' => Copy,
            _ => {
                eprintln!("bad mask bit: {}", c);
                panic!();
            }
        }
    }
}

impl From<usize> for Bit {
    fn from(c: usize) -> Self {
        use Bit::*;

        match c {
            0 => Zero,
            1 => One,
            _ => {
                eprintln!("bad mask bit: {}", c);
                panic!();
            }
        }
    }
}

impl Bit {
    fn or(&self, other: &Self) -> Self {
        match *other {
            Bit::Zero => Bit::Zero,
            Bit::One => Bit::One,
            Bit::Copy => *self,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Mask {
    mask: [Bit; 36],
}

impl From<usize> for Mask {
    fn from(mut num: usize) -> Self {
        let mut mask = [Bit::Zero; 36];
        let mut i = 0;

        while num > 0 {
            let bit = Bit::from(num % 2);
            mask[i] = bit;

            num >>= 1;
            i += 1;
        }

        Mask { mask }
    }
}

impl From<&str> for Mask {
    fn from(string: &str) -> Self {
        let mut mask = [Bit::Zero; 36];
        let mut ugh = string.chars().map(Bit::from).rev();

        for i in 0..36 {
            mask[i] = ugh.next().unwrap();
        }

        Mask { mask }
    }
}

impl From<&Mask> for usize {
    fn from(mask: &Mask) -> Self {
        let mut total = 0;
        let mut multiplier = 1;

        for i in 0..36 {
            match mask.mask[i] {
                Bit::Zero => (),
                Bit::One => total += 1 * multiplier,
                Bit::Copy => panic!(),
            }

            multiplier <<= 1;
        }

        total
    }
}

impl Mask {
    fn or(&self, other: &Self) -> Self {
        let mut ugh = self
            .mask
            .iter()
            .zip(other.mask.iter())
            .map(|(left, right)| left.or(&right));

        let mut mask = [Bit::Zero; 36];
        for i in 0..36 {
            mask[i] = ugh.next().unwrap();
        }

        Self { mask }
    }
}

fn main() {
    part1();
}

fn part1() {
    let v: Vec<(&str, &str)> = INPUT
        .trim()
        .lines()
        .map(|line| line.split(" = "))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .collect();

    let mask = [Bit::Zero; 36];
    let mut mask = Mask { mask };
    let mut memory = [mask.clone(); 65_536];

    for (left, right) in v.iter() {
        if left.starts_with("mask") {
            mask = Mask::from(*right);
        } else if left.starts_with("mem") {
            let bounds = (left.find('[').unwrap() + 1, left.find(']').unwrap());
            let address = &left[bounds.0..bounds.1];
            let address: usize = address.parse().unwrap();

            let value: usize = right.parse().unwrap();
            let value_mask = Mask::from(value);
            let new_value = value_mask.or(&mask);

            memory[address] = new_value;
        }
    }

    let sum: usize = memory.iter().map(usize::from).sum();
    println!("part1 = {}", sum);
}
