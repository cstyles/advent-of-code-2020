use std::collections::{HashMap, VecDeque};
use std::convert::From;
use std::fmt;

static INPUT: &str = include_str!("../input.txt");
// static INPUT: &str = include_str!("../test-input.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl From<Bit> for char {
    fn from(bit: Bit) -> Self {
        use Bit::*;

        match bit {
            Zero => '0',
            One => '1',
            Copy => 'X',
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

    fn or2(&self, other: &Self) -> Self {
        match *other {
            Bit::Zero => *self,
            Bit::One => Bit::One,
            Bit::Copy => Bit::Copy,
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

        for bit in mask.iter_mut() {
            *bit = ugh.next().unwrap();
        }

        Mask { mask }
    }
}

impl From<Mask> for usize {
    fn from(mask: Mask) -> Self {
        let mut total = 0;
        let mut multiplier = 1;

        for i in 0..36 {
            match mask.mask[i] {
                Bit::Zero => (),
                Bit::One => total += multiplier,
                Bit::Copy => panic!(),
            }

            multiplier <<= 1;
        }

        total
    }
}

impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ugh: String = self.mask.iter().map(|&bit| char::from(bit)).collect();

        write!(f, "M({})", ugh)
    }
}

impl Mask {
    fn or(&self, other: &Self) -> Self {
        let mut mask = [Bit::Zero; 36];
        for (i, bit) in mask.iter_mut().enumerate() {
            *bit = self.mask[i].or(&other.mask[i]);
        }

        Self { mask }
    }

    fn or2(&self, other: &Self) -> Self {
        let mut mask = [Bit::Zero; 36];
        for (i, bit) in mask.iter_mut().enumerate() {
            *bit = self.mask[i].or2(&other.mask[i]);
        }

        Self { mask }
    }
}

fn main() {
    part1();
    part2();
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
    let mut memory = [mask; 65_536];

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

    let sum: usize = memory.iter().map(|mask| usize::from(*mask)).sum();
    println!("part1 = {}", sum);
}

fn part2() {
    let v: Vec<(&str, &str)> = INPUT
        .trim()
        .lines()
        .map(|line| line.split(" = "))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .collect();

    let mask = [Bit::Zero; 36];
    let mut mask = Mask { mask };
    let mut memory: HashMap<usize, Mask> = Default::default();

    for (left, right) in v.iter() {
        if left.starts_with("mask") {
            mask = Mask::from(*right);
        } else if left.starts_with("mem") {
            let bounds = (left.find('[').unwrap() + 1, left.find(']').unwrap());
            let address = &left[bounds.0..bounds.1];
            let address: usize = address.parse().unwrap();
            let address_mask = Mask::from(address);

            let value: usize = right.parse().unwrap();
            let value_mask = Mask::from(value);

            let mut done: Vec<Mask> = vec![];
            let mut masks: VecDeque<Mask> = Default::default();
            let initial_address_mask = address_mask.or2(&mask);
            masks.push_back(initial_address_mask);

            while !masks.is_empty() {
                let mut mask = masks.pop_front().unwrap();
                let copy_index = match mask.mask.iter().position(|&bit| bit == Bit::Copy) {
                    Some(i) => i,
                    None => {
                        done.push(mask);
                        continue;
                    }
                };

                let mut one_mask = mask;
                one_mask.mask[copy_index] = Bit::One;
                masks.push_back(one_mask);

                mask.mask[copy_index] = Bit::Zero;
                masks.push_back(mask);
            }

            for mask in done {
                let address = usize::from(mask);
                memory.insert(address, value_mask);
            }
        }
    }

    let sum: usize = memory
        .iter()
        .map(|(_address, mask)| usize::from(*mask))
        .sum();
    println!("part2 = {}", sum);
}
