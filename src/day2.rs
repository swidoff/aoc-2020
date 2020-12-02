use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug)]
struct PasswordRecord {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PasswordRecord {
    fn is_valid_part1(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.letter).count();
        count >= self.min && count <= self.max
    }

    fn is_valid_part2(&self) -> bool {
        let c1 = self.char_at(self.min - 1);
        let c2 = self.char_at(self.max - 1);
        (c1 == self.letter) ^ (c2 == self.letter)
    }

    fn char_at(&self, index: usize) -> char {
        (&self.password[index..]).chars().nth(0).unwrap()
    }
}

fn read_password_database() -> Vec<PasswordRecord> {
    let file = File::open("input/day2.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let min_max_part = parts.next().unwrap();
            let letter_part = parts.next().unwrap();
            let password = parts.next().unwrap();
            let (min_count_str, max_count_str) =
                min_max_part.split_at(min_max_part.find("-").unwrap());

            PasswordRecord {
                min: usize::from_str(min_count_str).unwrap(),
                max: usize::from_str(&max_count_str[1..]).unwrap(),
                letter: letter_part.chars().next().unwrap(),
                password: password.to_string(),
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::day2::read_password_database;

    #[test]
    fn part1() {
        let db = read_password_database();
        let valid_count = db.iter().filter(|r| r.is_valid_part1()).count();
        println!("{}", valid_count);
    }

    #[test]
    fn part2() {
        let db = read_password_database();
        let valid_count = db.iter().filter(|r| r.is_valid_part2()).count();
        println!("{}", valid_count);
    }
}
