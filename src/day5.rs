use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn read_file() -> Vec<String> {
    let mut file = File::open("input/day5.txt").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    str.lines().map(|line| line.to_string()).collect_vec()
}

#[derive(Debug)]
struct BoardingPass {
    row: u32,
    column: u32,
}

impl BoardingPass {
    fn seat(&self) -> u32 {
        self.row * 8 + self.column
    }
}

fn decode_pass(pass: &String) -> BoardingPass {
    let (row_codes, column_codes) = pass.split_at(7);
    let row = locate_position(128, row_codes, 'F');
    let column = locate_position(8, column_codes, 'L');
    BoardingPass { row, column }
}

fn locate_position(positions: u32, codes: &str, lower_code: char) -> u32 {
    let mut min = 0;
    let mut max = positions;
    let (first_codes, last_code) = codes.split_at(codes.len() - 1);

    for c in first_codes.chars() {
        let increment = (max - min) / 2;
        if c == lower_code {
            max -= increment;
        } else {
            min += increment;
        }
    }

    match last_code.chars().nth(0) {
        Some(c) if c == lower_code => min,
        _ => max - 1,
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::{decode_pass, read_file};
    use itertools::Itertools;

    #[test]
    fn part1_example() {
        assert_eq!(decode_pass(&"FBFBBFFRLR".to_string()).seat(), 357);
        assert_eq!(decode_pass(&"BFFFBBFRRR".to_string()).seat(), 567);
        assert_eq!(decode_pass(&"FFFBBBFRRR".to_string()).seat(), 119);
        assert_eq!(decode_pass(&"BBFFBBFRLL".to_string()).seat(), 820);
    }

    #[test]
    fn part1() {
        let passes = read_file();
        let res = passes.iter().map(|l| decode_pass(l).seat()).max();
        println!("{}", res.unwrap());
    }

    #[test]
    fn part2() {
        let passes = read_file();
        let mut seats = passes.iter().map(|l| decode_pass(l).seat()).collect_vec();
        seats.sort();

        let mut missing = Vec::new();

        // Visual Method (the one I actually used).

        println!("1 2 3 4 5 6 7 8");
        for row in 0..127 {
            for col in 0..8 {
                let seat = row * 8 + col;
                if seats.binary_search(&seat).is_ok() {
                    print!("X ");
                } else {
                    print!("  ");
                    missing.push(seat);
                }
            }
            println!(" {}", row);
        }

        // Actual computation method.

        let res = missing
            .iter()
            .skip(1)
            .find(|v| {
                seats.binary_search(&(**v - 1)).is_ok() && seats.binary_search(&(**v + 1)).is_ok()
            })
            .unwrap();
        println!("{}", res);
        assert_eq!(*res, 657);
    }
}
