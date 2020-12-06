use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day6.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|s| s.unwrap().to_string())
        .chain("\n".lines().map(|l| l.to_string())) // Add an extra new line at the end.
}

fn sum_answer_counts(lines: impl Iterator<Item = String>, union: bool) -> u32 {
    let initial_set: u32 = match union {
        true => u32::min_value(),
        false => u32::max_value(),
    };

    let (count, _) = lines.fold((0, initial_set), |(count, set), line| {
        if line.is_empty() {
            (count + set.count_ones(), initial_set)
        } else {
            let mut line_set = 0;
            for c in line.bytes() {
                line_set |= 1 << (25 - (122 - c)) // Convert letter to bit index. 122 == 'z'
            }

            let new_set = match union {
                true => set | line_set,
                false => set & line_set,
            };
            (count, new_set)
        }
    });

    count
}

#[cfg(test)]
mod tests {
    use crate::day6::{read_file, sum_answer_counts};

    const EXAMPLE: &str = "
abc

a
b
c

ab
ac

a
a
a
a

b

";

    #[test]
    fn part1_example() {
        let res = sum_answer_counts(
            EXAMPLE[1..].to_string().lines().map(|s| s.to_string()),
            true,
        );
        println!("{}", res);
        assert_eq!(res, 11);
    }

    #[test]
    fn part1() {
        let res = sum_answer_counts(read_file(), true);
        println!("{}", res);
        assert_eq!(res, 6903);
    }

    #[test]
    fn part2_example() {
        let res = sum_answer_counts(
            EXAMPLE[1..].to_string().lines().map(|s| s.to_string()),
            false,
        );
        println!("{}", res);
        assert_eq!(res, 6);
    }

    #[test]
    fn part2() {
        let res = sum_answer_counts(read_file(), false);
        println!("{}", res);
        assert_eq!(res, 3493);
    }
}
