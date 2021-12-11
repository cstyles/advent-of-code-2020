use std::collections::VecDeque;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Label(u32);

impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

struct Circle {
    cups: VecDeque<Label>,
    current: usize,
}

impl Circle {
    pub fn new(cups: Vec<Label>) -> Self {
        Self {
            cups: VecDeque::from(cups),
            current: 0,
        }
    }

    fn step(&mut self) {
        // Step 0: Rotate until current cup is on far "left"
        self.cups.rotate_left(self.current);
        self.current = 0;

        // println!("cups: {:?}", self.cups);
        // print!("       ");
        // for _ in 0..self.current {
        //     print!("   ");
        // }
        // println!("^");

        // Step 1: Remove three cups
        let start = self.current.wrapping_next();
        let end = start.wrapping_next().wrapping_next().wrapping_next();
        let mut picked_up: VecDeque<Label> = self.cups.drain(start..end).collect();
        // println!("pick up: {:?}", picked_up);

        // Step 2: Find destination
        let current_label: Label = self.cups[self.current];
        let mut destination_label: Label = current_label.wrapping_prev();
        while picked_up.contains(&destination_label) {
            destination_label = destination_label.wrapping_prev();
        }
        // dbg!(destination_label);

        let destination = self
            .cups
            .iter()
            .position(|&cup| cup == destination_label)
            .unwrap()
            .wrapping_next();
        // dbg!(destination);

        // Step 3: Place picked up cups at destination
        let mut right = self.cups.split_off(destination);
        self.cups.append(&mut picked_up);
        self.cups.append(&mut right);

        // Compensate when picked up cups were placed to the "left" of the current cup
        if destination <= self.current {
            self.current = self.current.wrapping_next().wrapping_next().wrapping_next();
        }

        // Step 4: Select new current cup
        self.current = self.current.wrapping_next();
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let numbers: Vec<Label> = input
        .trim()
        .chars()
        .map(|c| Label(c.to_digit(10).unwrap()))
        .collect();
    let mut circle = Circle::new(numbers);

    for i in 1..=100 {
        // println!("-- move {} --", i);
        circle.step();
        // println!();
    }

    let one_index = circle
        .cups
        .iter()
        .position(|&Label(label)| label == 1)
        .unwrap();

    circle.cups.rotate_left(one_index);

    let part1: String = circle
        .cups
        .into_iter()
        .skip(1)
        .map(|Label(label)| char::from_digit(label, 10).unwrap())
        .collect();

    println!("part1 = {}", part1);
}

trait Wrapping {
    fn wrapping_next(&self) -> Self;
    fn wrapping_prev(&self) -> Self;
}

impl Wrapping for Label {
    fn wrapping_next(&self) -> Self {
        let sum = self.0 + 1;

        if sum > 9 {
            Label(1)
        } else {
            Label(sum)
        }
    }

    fn wrapping_prev(&self) -> Self {
        if self.0 == 1 {
            Label(9)
        } else {
            Label(self.0 - 1)
        }
    }
}

impl Wrapping for usize {
    fn wrapping_next(&self) -> Self {
        let sum = *self + 1;

        if sum > 9 {
            0
        } else {
            sum
        }
    }

    fn wrapping_prev(&self) -> Self {
        if *self == 0 {
            9
        } else {
            *self - 1
        }
    }
}
