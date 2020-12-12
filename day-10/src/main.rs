static INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut adapters: Vec<i64> = INPUT.lines().map(|line| line.parse().unwrap()).collect();

    adapters.sort();

    // println!("{:#?}", adapters);

    let diff = part1(&adapters);
    println!("part1 = {}", diff);
}

fn part1(adapters: &[i64]) -> i64 {
    let initial_state = match adapters[0] {
        1 => (1, 0),
        3 => (0, 1),
        _ => (0, 0),
    };

    // println!("initial_state = {:?}", initial_state);

    let (one_jolt_diffs, three_jolt_diffs) = adapters.windows(2).fold(
        initial_state,
        |(one_jolt_diffs, three_jolt_diffs), slice| {
            // println!("1: {}; 3: {}; slice: {:?}, diff: {}", one_jolt_diffs, three_jolt_diffs, slice, slice[1] - slice[0]);

            match slice[1] - slice[0] {
                1 => (one_jolt_diffs + 1, three_jolt_diffs),
                3 => (one_jolt_diffs, three_jolt_diffs + 1),
                _ => (one_jolt_diffs, three_jolt_diffs),
            }
        });

    one_jolt_diffs * (three_jolt_diffs + 1)
}
