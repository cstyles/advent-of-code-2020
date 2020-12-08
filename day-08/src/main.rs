use std::collections::HashSet;
use std::convert::From;

static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Instruction {
    fn flop(self) -> Self {
        use Instruction::*;

        match self {
            Nop(offset) => Jmp(offset),
            Jmp(offset) => Nop(offset),
            Acc(_) => {
                eprintln!("can't flop Acc");
                panic!();
            } // x => x,
        }
    }
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let opcode = &string[..3];
        let offset = string[4..].parse().unwrap();

        match opcode {
            "nop" => Self::Nop(offset),
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
    let code: Vec<Instruction> = INPUT.lines().map(Instruction::from).collect();

    part1(&code);
    part2(code);
}

fn part1(code: &[Instruction]) {
    let (_, acc) = run_program(code);
    println!("part1: {}", acc);
}

// return type: (terminated?, acc_at_end)
fn run_program(code: &[Instruction]) -> (bool, isize) {
    use Instruction::*;

    let mut pc: isize = 0;
    let mut acc: isize = 0;
    let mut visited_instructions: HashSet<usize> = Default::default();

    loop {
        if visited_instructions.contains(&(pc as usize)) {
            return (false, acc);
        } else {
            visited_instructions.insert(pc as usize);
        }

        if pc as usize == code.len() {
            return (true, acc);
        }

        visited_instructions.insert(pc as usize);

        let instruction = code.get(pc as usize).unwrap();
        match instruction {
            Nop(_) => (),
            Acc(offset) => acc += offset,
            Jmp(offset) => pc += offset - 1,
        }

        pc += 1;
    }
}

fn part2(mut code: Vec<Instruction>) {
    for i in 0..code.len() {
        if let Instruction::Acc(_) = code[i] {
            continue;
        }

        code[i] = code[i].flop();
        let (terminated, acc) = run_program(&code);

        if terminated {
            println!("part2: ({}, {})", i, acc);
        }

        code[i] = code[i].flop();
    }
}
