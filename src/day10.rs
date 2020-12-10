use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::vec::Vec;

fn read_file() -> impl Iterator<Item = u64> {
    let file = File::open("input/day10.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|s| u64::from_str(s.unwrap().as_str()).unwrap())
}

fn joltage_distribution(adapters: &mut Vec<u64>) -> (u64, u64) {
    // Include zero and your final adapter.
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort();

    // Since we know the distance is never greater than three, just count the distances between
    // adjacent adapters.
    adapters
        .iter()
        .tuple_windows()
        .fold((0, 0), |(ones, threes), (v1, v2)| match v2 - v1 {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            2 => (ones, threes),
            x => panic!("Unexpected difference: {}", x),
        })
}

fn count_arrangements(adapters: &mut Vec<u64>) -> u64 {
    // Include zero and your final adapter.
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort();

    // Use bottom-up dynamic programming to avoid recalculating overlapping sub-problems.
    // Sum the counts including and excluding each previous adapter within a distance of 3.
    // You really only need up to three counts, but there are only 100 rows in the file.
    let mut counts: Vec<u64> = Vec::with_capacity(adapters.len());
    counts.push(1);
    for i in 1..adapters.len() {
        let adapter = adapters[i];
        let mut j: i32 = i as i32 - 1;

        let mut count = 0;
        while j >= 0 && adapter - adapters[j as usize] <= 3 {
            count += counts[j as usize];
            j -= 1;
        }

        counts.push(count);
    }

    counts[adapters.len() - 1]
}

#[cfg(test)]
mod tests {
    use crate::day10::{count_arrangements, joltage_distribution, read_file};
    use itertools::Itertools;

    #[test]
    fn part1_example1() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let res = joltage_distribution(&mut adapters);
        assert_eq!((7, 5), res)
    }

    #[test]
    fn part1_example2() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let res = joltage_distribution(&mut adapters);
        assert_eq!((22, 10), res)
    }

    #[test]
    fn part1() {
        let mut adapters = read_file().collect_vec();
        let (ones, threes) = joltage_distribution(&mut adapters);
        let product = ones * threes;
        println!("{}", product);
        assert_eq!(2590, product);
    }

    #[test]
    fn part2_example1() {
        let mut adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let res = count_arrangements(&mut adapters);
        assert_eq!(res, 8);
    }

    #[test]
    fn part2_example2() {
        let mut adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let res = count_arrangements(&mut adapters);
        assert_eq!(res, 19208);
    }

    #[test]
    fn part2() {
        let mut adapters = read_file().collect_vec();
        let res = count_arrangements(&mut adapters);
        println!("{}", res);
        assert_eq!(226775649501184, res);
    }
}
