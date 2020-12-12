use std::collections::HashMap;

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut adapters: Vec<i64> = INPUT.lines().map(|line| line.parse().unwrap()).collect();

    adapters.sort();

    let diff = part1(&adapters);
    println!("part1 = {}", diff);

    let number_of_arrangements = part2(&adapters);
    println!("part2 = {}", number_of_arrangements);
}

fn part1(adapters: &[i64]) -> i64 {
    let initial_state = match adapters[0] {
        1 => (1, 0),
        3 => (0, 1),
        _ => (0, 0),
    };

    let (one_jolt_diffs, three_jolt_diffs) = adapters.windows(2).fold(
        initial_state,
        |(one_jolt_diffs, three_jolt_diffs), slice| match slice[1] - slice[0] {
            1 => (one_jolt_diffs + 1, three_jolt_diffs),
            3 => (one_jolt_diffs, three_jolt_diffs + 1),
            _ => (one_jolt_diffs, three_jolt_diffs),
        },
    );

    one_jolt_diffs * (three_jolt_diffs + 1)
}

fn part2(adapters: &[i64]) -> i64 {
    let mut dyn_prog: HashMap<usize, i64> = HashMap::with_capacity(adapters.len());
    dyn_prog.insert(0, 1);

    ways(adapters, &mut dyn_prog, adapters.len() - 1)
}

fn ways(adapters: &[i64], dyn_prog: &mut HashMap<usize, i64>, cursor: usize) -> i64 {
    if dyn_prog.contains_key(&cursor) {
        let rt = *dyn_prog.get(&cursor).unwrap();
        // println!("ways down from {} = {}", adapters[cursor], rt);
        rt
    } else {
        let adapter = adapters[cursor];
        let mut blargh = [None, None, None];

        for i in 1..=3 {
            let next_cursor = match cursor.checked_sub(i) {
                Some(x) => x,
                None => {
                    blargh[3 - i] = Some(1);
                    break;
                }
            };

            if adapter - adapters[next_cursor] <= 3 {
                blargh[3 - i] = Some(ways(adapters, dyn_prog, next_cursor));
            }
        }

        // println!("adapter = {}; blargh = {:?}", adapter, blargh);
        let rt = blargh.iter().filter_map(|x| *x).sum();
        dyn_prog.insert(cursor, rt);
        rt
    }
}
