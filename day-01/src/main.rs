static INPUT: &str = include_str!("../input.txt");

fn main() {
    let numbers: Vec<i32> = INPUT.trim().lines().flat_map(|line| line.parse()).collect();

    let size = numbers.len();

    part1(&numbers, size);
    part2(&numbers, size);
}

fn part1(numbers: &[i32], size: usize) {
    for i in 0..size {
        for j in 0..(i + 1) {
            let sum = numbers[i] + numbers[j];
            if sum == 2020 {
                let product = numbers[i] * numbers[j];
                println!("i: {}, j: {}, product: {}", i, j, product);
            }
        }
    }
}

fn part2(numbers: &[i32], size: usize) {
    for i in 0..size {
        for j in 0..(i + 1) {
            let partial_sum = numbers[i] + numbers[j];
            if partial_sum >= 2020 {
                continue;
            }

            for k in 0..(i + 1) {
                let sum = partial_sum + numbers[k];
                if sum == 2020 {
                    let product = numbers[i] * numbers[j] * numbers[k];
                    println!("i: {}, j: {}, k: {}, product: {}", i, j, k, product);
                }
            }
        }
    }
}
