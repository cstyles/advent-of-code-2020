fn main() {
    part1();
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
        // println!("round_number = {}", round_number);
        let last_number = numbers[round_number];
        // println!("last_number = {}", last_number);

        let i = numbers
            .iter()
            .rev()
            .skip(1)
            .position(|num| *num == last_number);

        match i {
            None => {
                // println!("first time encountered, pushing 0");
                numbers.push(0)
            }
            Some(i) => {
                // println!("encountered {} numbers ago", i);
                numbers.push(i + 1); // need to add 1 b/c skip(1)
            }
        }
        // println!();
    }

    println!("part1 = {}", numbers[2019]); // 1 less to account for 1-based indexing
}
