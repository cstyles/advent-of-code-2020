static INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
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
