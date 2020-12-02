use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

/// Assuming file does not contain duplicates (mine does not)
/// Assuming the sum does not pair (or triple) a number with itself (1010 is not in the file)

fn read_file_as_set() -> HashSet<u32> {
    let file = File::open("input/day1.txt").unwrap();
    HashSet::from_iter(
        BufReader::new(file)
            .lines()
            .map(|line| u32::from_str(line.unwrap().as_str()).unwrap()),
    )
}

fn find_pair_with_sum(input: &HashSet<u32>, sum: u32) -> Option<(u32, u32)> {
    input
        .iter()
        .filter(|v| **v < sum)
        .find_map(|v1| input.get(&(sum - *v1)).map(|v2| (*v1, *v2)))
}

fn find_triple_with_sum(input: &HashSet<u32>, sum: u32) -> Option<(u32, u32, u32)> {
    input
        .iter()
        .filter(|v| **v < sum)
        .find_map(|v1| find_pair_with_sum(input, sum - *v1).map(|(v2, v3)| (*v1, v2, v3)))
}

#[cfg(test)]
mod tests {
    use crate::day1::{find_pair_with_sum, find_triple_with_sum, read_file_as_set};

    #[test]
    fn part1() {
        let input = read_file_as_set();
        match find_pair_with_sum(&input, 2020) {
            Some((v1, v2)) => println!("{}", v1 * v2),
            None => println!("No solution found"),
        }
    }

    #[test]
    fn part2() {
        let input = read_file_as_set();
        match find_triple_with_sum(&input, 2020) {
            Some((v1, v2, v3)) => println!("{}", v1 * v2 * v3),
            None => println!("No solution found"),
        }
    }
}
