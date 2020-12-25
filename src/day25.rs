use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day25.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

fn parse_keys(iter: impl Iterator<Item = String>) -> (u64, u64) {
    iter.map(|line| u64::from_str(&line).unwrap())
        .collect_tuple()
        .unwrap()
}

fn transform(subject_number: u64, loop_size: usize) -> impl Iterator<Item = (u64, usize)> {
    (1..(loop_size + 1)).scan(1, move |value, i| {
        *value *= subject_number;
        *value %= 20201227;
        Some((*value, i))
    })
}

fn find_encryption_key(door_key: u64, card_key: u64, loops: usize) -> u64 {
    let (other_key, loop_size) = transform(7, loops)
        .find_map(|(key, loop_size)| {
            if key == door_key {
                Some((card_key, loop_size))
            } else if key == card_key {
                Some((door_key, loop_size))
            } else {
                None
            }
        })
        .unwrap();

    transform(other_key, loop_size).last().unwrap().0
}

#[cfg(test)]
mod tests {
    use crate::day25::{find_encryption_key, parse_keys, read_file, transform};

    #[test]
    fn test_part1_example() {
        assert_eq!(5764801, transform(7, 8).last().unwrap().0);
        assert_eq!(17807724, transform(7, 11).last().unwrap().0);
        assert_eq!(14897079, transform(5764801, 11).last().unwrap().0);
        assert_eq!(14897079, transform(17807724, 8).last().unwrap().0);
        assert_eq!(14897079, find_encryption_key(17807724, 5764801, 20));
    }

    #[test]
    fn test_part1() {
        let (door_key, card_key) = parse_keys(read_file());
        let res = find_encryption_key(door_key, card_key, 10000000);
        println!("{}", res);
        assert_eq!(, res);
    }
}
