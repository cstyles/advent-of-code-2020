#![feature(array_windows)]

#[derive(Debug)]
struct LinkedList<const N: usize> {
    cups: [u32; N],
    current: usize,
}

impl<const N: usize> LinkedList<N> {
    pub fn new(numbers: &[u32]) -> Self {
        let mut array = [0; N];
        for [a, b] in numbers.array_windows().copied() {
            array[a as usize] = b;
        }

        // Account for last number that wraps around and points to the first number
        let first = *numbers.first().unwrap();
        array[*numbers.last().unwrap() as usize] = first;

        Self {
            cups: array,
            current: first as usize,
        }
    }

    pub fn step(&mut self) {
        // println!("cups: {:?}", self.cups);

        let plucked = self.pluck();
        // println!("pick up: {:?}", plucked);

        let destination = self.find_destination(plucked);
        // dbg!(destination);

        self.place(plucked, destination);
        self.next_cup();
    }

    // Pick up three cups and adjust cup spacing
    pub fn pluck(&mut self) -> [u32; 3] {
        let mut plucked = [0; 3];
        plucked[0] = self.cups[self.current];
        plucked[1] = self.cups[plucked[0] as usize]; // TODO: unnecessary
        plucked[2] = self.cups[plucked[1] as usize];

        self.cups[self.current] = self.cups[plucked[2] as usize];

        plucked
    }

    fn find_destination(&self, plucked: [u32; 3]) -> usize {
        let mut destination = wrapping_prev::<N>(self.current);

        // Skip over plucked cups if necessary
        while plucked.contains(&(destination as u32)) {
            destination = wrapping_prev::<N>(destination);
        }

        destination
    }

    fn place(&mut self, plucked: [u32; 3], destination: usize) {
        let old_target = self.cups[destination];
        self.cups[destination] = plucked[0];
        self.cups[plucked[2] as usize] = old_target;
    }

    fn next_cup(&mut self) {
        self.current = self.cups[self.current] as usize;
    }

    fn order(&self) -> String {
        let mut start = self.cups[1];
        let mut string = String::with_capacity(9);

        while start != 1 {
            let c = char::from_digit(start, 10).unwrap();
            string.push(c);

            start = self.cups[start as usize];
        }

        string
    }

    fn star_product(&self) -> u64 {
        let a = self.cups[1] as u64;
        let b = self.cups[a as usize] as u64;

        a * b
    }
}

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let numbers: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut list = LinkedList::<10>::new(&numbers);

    for _i in 1..=100 {
        // println!("-- move {} --", i);
        list.step();
        // println!();
    }

    println!("part1 = {}", list.order());
}

fn part2(input: &str) {
    let mut numbers: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    for i in (numbers.iter().max().unwrap() + 1)..=1_000_000 {
        numbers.push(i);
    }

    let mut list = LinkedList::<1_000_001>::new(&numbers);

    for _i in 1..=10_000_000 {
        // println!("-- move {} --", i);
        list.step();
        // println!();
    }

    println!("part2 = {}", list.star_product());
}

fn wrapping_prev<const N: usize>(lhs: usize) -> usize {
    if lhs == 1 {
        N - 1
    } else {
        lhs - 1
    }
}
