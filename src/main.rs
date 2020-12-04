fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let numbers: Vec<i32> = input
        .trim()
        .split("\n")
        .flat_map(|line| line.parse())
        .collect();

    let size = numbers.len();

    for i in 0..size {
        for j in 0..size {
            let sum = numbers[i] + numbers[j];
            if sum == 2020 {
                let product = numbers[i] * numbers[j];
                println!("i: {}, j: {}, product: {}", i, j, product);
            }
        }
    }
}