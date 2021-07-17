static INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut lines = INPUT.lines();
    let timestamp: usize = lines.next().unwrap().parse().unwrap();
    let (bus_id, until): (usize, usize) = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|id| id.parse::<usize>().ok())
        .map(|bus| {
            let just_before = timestamp / bus * bus;
            let next_bus = just_before + bus;
            (bus, next_bus - timestamp)
        })
        .min_by(|&x, &y| x.1.cmp(&y.1))
        .unwrap();

    println!("part1 = {}", bus_id * until);
}

fn part2() {
    let mut pairs: Vec<(usize, usize)> = INPUT
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, id)| match id.parse() {
            Ok(id) => Some((i, id)),
            Err(_err) => None,
        })
        .collect();

    pairs.sort_by(|x, y| y.1.cmp(&x.1));

    let mut ts = pairs[0].1 - pairs[0].0;

    'outer: loop {
        let mut jump_by = pairs[0].1;

        for (i, id) in pairs.iter().skip(1) {
            if (ts + i) % id != 0 {
                ts += jump_by;
                continue 'outer;
            } else {
                jump_by *= id;
            }
        }

        break;
    }

    println!("part2 = {}", ts);
}
