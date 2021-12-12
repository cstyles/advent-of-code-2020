fn generate_key(subject_number: u64, loop_size: usize) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }

    value
}

fn find_loop_size(subject_number: u64, key: u64) -> usize {
    let mut loop_size = 0;
    let mut value = 1;

    loop {
        if value == key {
            return loop_size;
        }

        loop_size += 1;
        value *= subject_number;
        value %= 20201227;
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let keys: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let card_pub_key = keys[0];
    let door_pub_key = keys[1];

    let card_loop_size = find_loop_size(7, card_pub_key);
    let door_loop_size = find_loop_size(7, door_pub_key);

    let encryption_key = generate_key(card_pub_key, door_loop_size);
    assert_eq!(encryption_key, generate_key(door_pub_key, card_loop_size));
    println!("part1 = {}", encryption_key);
}

mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    fn generate_key_test() {
        assert_eq!(5764801, generate_key(7, 8));
        assert_eq!(17807724, generate_key(7, 11));

        assert_eq!(14897079, generate_key(5764801, 11));
        assert_eq!(14897079, generate_key(17807724, 8));
    }
}
