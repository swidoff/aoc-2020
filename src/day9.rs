use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::vec::Vec;

fn read_file() -> impl Iterator<Item = u64> {
    let file = File::open("input/day9.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|s| u64::from_str(s.unwrap().as_str()).unwrap())
}

/// An invalid entry is one for which there does not exist a pair of values that sum to it in the preceding `preamble`
/// entries.
fn find_first_invalid(seq: &Vec<u64>, preamble: usize) -> Option<u64> {
    (preamble..seq.len()).find_map(|index| {
        let value = seq[index];
        if is_sum_of_pair(value, &seq[index - preamble..index]) {
            None
        } else {
            Some(value)
        }
    })
}

/// Returns true if there are a pair of values in `candidates` that sum to `value.
fn is_sum_of_pair(value: u64, candidates: &[u64]) -> bool {
    // Simple n^2 algorithm works just fine for inputs of this length: n == 25
    candidates
        .iter()
        .filter(|v| **v < value)
        .map(|v| value - v)
        .enumerate()
        .any(|(i, expected)| candidates[i + 1..].iter().any(|v2| *v2 == expected))
}

/// Returns the min and max values in any contiguous subsequence of `candidates` that sums to `target`.
fn find_summing_subsequence(target: u64, candidates: &Vec<u64>) -> Option<(u64, u64)> {
    // Simple n^2 algorithm works just fine for inputs of this length.
    for i in 0..candidates.len() {
        let mut sum = 0;
        let mut j = i;
        let mut min = u64::max_value();
        let mut max = u64::min_value();
        while sum < target && j < candidates.len() {
            let candidate = candidates[j];
            sum += candidate;
            min = candidate.min(min);
            max = candidate.max(max);
            j += 1;
        }

        if sum == target {
            return Some((min, max));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::day9::{find_first_invalid, find_summing_subsequence, read_file};
    use itertools::Itertools;

    #[test]
    fn part1_example() {
        let vec = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let res = find_first_invalid(&vec, 5);
        assert_eq!(res, Some(127));
    }

    #[test]
    fn part1() {
        let vec = read_file().collect_vec();
        let res = find_first_invalid(&vec, 25);
        println!("{}", res.unwrap());
        assert_eq!(res, Some(177777905));
    }

    #[test]
    fn part2_example() {
        let vec = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let res = find_summing_subsequence(127, &vec);
        assert_eq!(res, Some((15, 47)));
    }

    #[test]
    fn part2() {
        let vec = read_file().collect_vec();
        if let Some((min, max)) = find_summing_subsequence(177777905, &vec) {
            println!("{} {} {}", min, max, min + max);
            assert_eq!(23463012, min + max);
        } else {
            panic!("Answer not found");
        }
    }
}
