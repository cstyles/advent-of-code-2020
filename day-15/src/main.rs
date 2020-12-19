use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut numbers = vec![0, 13, 1, 8, 6, 15];
    // let mut numbers = vec![0, 3, 6];
    // let mut numbers = vec![1, 3, 2];
    // let mut numbers = vec![2, 1, 3];
    // let mut numbers = vec![1, 2, 3];
    // let mut numbers = vec![2, 3, 1];
    // let mut numbers = vec![3, 2, 1];
    // let mut numbers = vec![3, 1, 2];

    numbers.push(0);
    let start = numbers.len() - 1;
    // println!("{:?}", numbers);

    for round_number in start..2020 {
        let last_number = numbers[round_number];
        // println!(
        //     "round_number = {}; last_number = {}",
        //     round_number, last_number
        // );

        let i = numbers
            .iter()
            .rev()
            .skip(1)
            .position(|num| *num == last_number);

        match i {
            None => {
                // println!("{} hasn't been said", last_number);
                numbers.push(0)
            }
            Some(i) => {
                // println!("{} encountered {} rounds ago", last_number, i + 1);
                numbers.push(i + 1); // need to add 1 b/c skip(1)
            }
        }
        // println!();
    }

    println!("part1 = {}", numbers[2019]); // 1 less to account for 1-based indexing
}

fn part2() {
    let mut numbers: HashMap<usize, usize> = Default::default();
    // numbers.insert(0, 0);
    // numbers.insert(3, 1);

    numbers.insert(0, 0);
    numbers.insert(13, 1);
    numbers.insert(1, 2);
    numbers.insert(8, 3);
    numbers.insert(6, 4);

    let mut last_number = 15;

    for round_number in 5..29999999 {
        // println!(
        //     "round_number = {}; last_number = {}",
        //     round_number, last_number
        // );

        match numbers.get(&last_number) {
            None => {
                // println!("{} hasn't been said", last_number);
                numbers.insert(last_number, round_number);
                last_number = 0;
            }
            Some(last_encountered) => {
                let diff = round_number - last_encountered;
                // println!("{} was encountered {} rounds ago", last_number, diff);
                numbers.insert(last_number, round_number);
                last_number = diff;
            }
        }
        // println!();
    }

    println!("part2 = {}", last_number);
}
