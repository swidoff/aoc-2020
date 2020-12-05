use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

///
///  Initial version - Literally translates algorithm in the problem description.
///

fn read_file() -> Vec<String> {
    let file = File::open("input/day5.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().to_string())
        .collect_vec()
}

#[derive(Debug)]
struct BoardingPass {
    row: u16,
    column: u16,
}

impl BoardingPass {
    fn seat(&self) -> u16 {
        self.row * 8 + self.column
    }
}

fn decode_pass(pass: &String) -> BoardingPass {
    let (row_codes, column_codes) = pass.split_at(7);
    let row = locate_position(128, row_codes, 'F');
    let column = locate_position(8, column_codes, 'L');
    BoardingPass { row, column }
}

fn locate_position(positions: u16, codes: &str, lower_code: char) -> u16 {
    let mut min = 0;
    let mut max = positions;
    let (first_codes, last_code) = codes.split_at(codes.len() - 1);

    let mut increment = positions;
    for c in first_codes.chars() {
        increment >>= 1;
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

///
/// After the fact version. Maps letters directly to binary digits and lines to binary numbers.
///
/// Works entirely on iterators, but I believe the `lines` operator still does some dynamic memory allocation to
/// create the `String` objects it returns.
///

fn read_file_v2() -> impl Iterator<Item = u16> {
    let file = File::open("input/day5.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line_to_seat(&mut line.unwrap().chars()))
}

fn line_to_seat(line: &mut impl Iterator<Item = char>) -> u16 {
    line.map(|c| if c == 'F' || c == 'L' { 0 } else { 1 })
        .fold(0, |acc, bit| (acc << 1) | bit)
}

#[cfg(test)]
mod tests {
    use crate::day5::{decode_pass, line_to_seat, read_file, read_file_v2};
    use itertools::Itertools;

    #[test]
    fn part1_example() {
        assert_eq!(decode_pass(&"FBFBBFFRLR".to_string()).seat(), 357);
        assert_eq!(decode_pass(&"BFFFBBFRRR".to_string()).seat(), 567);
        assert_eq!(decode_pass(&"FFFBBBFRRR".to_string()).seat(), 119);
        assert_eq!(decode_pass(&"BBFFBBFRLL".to_string()).seat(), 820);
    }

    #[test]
    fn part1_example_v2() {
        assert_eq!(line_to_seat(&mut "FBFBBFFRLR".chars()), 357);
        assert_eq!(line_to_seat(&mut "BFFFBBFRRR".chars()), 567);
        assert_eq!(line_to_seat(&mut "FFFBBBFRRR".chars()), 119);
        assert_eq!(line_to_seat(&mut "BBFFBBFRLL".chars()), 820);
    }

    #[test]
    fn part1() {
        let passes = read_file();
        let res = passes.iter().map(|l| decode_pass(l).seat()).max();
        println!("{}", res.unwrap());
    }

    #[test]
    fn part1_v2() {
        let res = read_file_v2().max();
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

        // Computation method.

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
