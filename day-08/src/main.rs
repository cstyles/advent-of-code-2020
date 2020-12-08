use std::collections::HashSet;
use std::convert::From;

static INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
enum Instruction {
    Nop,
    Acc(isize),
    Jmp(isize),
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let opcode = &string[..3];
        let offset = string[4..].parse().unwrap();

        match opcode {
            "nop" => Self::Nop,
            "acc" => Self::Acc(offset),
            "jmp" => Self::Jmp(offset),
            _ => {
                eprintln!("invalid opcode: {}", opcode);
                panic!();
            }
        }
    }
}

fn main() {
    part1();
}

fn part1() {
    use Instruction::*;

    let mut pc: isize = 0;
    let mut acc: isize = 0;
    let code: Vec<Instruction> = INPUT.lines().map(Instruction::from).collect();
    let mut visited_instructions: HashSet<usize> = Default::default();

    loop {
        if visited_instructions.contains(&(pc as usize)) {
            break;
        } else {
            visited_instructions.insert(pc as usize);
        }

        visited_instructions.insert(pc as usize);

        let instruction = code.get(pc as usize).unwrap();
        // println!("pc: {}", pc);
        // println!("instruction: {:?}", instruction);
        match instruction {
            Nop => (),
            Acc(offset) => acc += offset,
            Jmp(offset) => pc += offset - 1,
        }

        pc += 1;
    }

    println!("part1: {}", acc);
}
