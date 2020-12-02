use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
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
        let slice = &self.password[(self.min - 1)..self.max];
        slice.starts_with(self.letter) ^ slice.ends_with(self.letter)
    }
}

fn read_password_database() -> impl Iterator<Item = PasswordRecord> {
    let file = File::open("input/day2.txt").unwrap();
    BufReader::new(file).lines().map(|line_res| {
        let line = line_res.unwrap();
        let mut parts = line.split_whitespace();
        let (min, max) = parts
            .next()
            .unwrap()
            .split("-")
            .map(|s| usize::from_str(s).unwrap())
            .collect_tuple()
            .unwrap();
        let letter = parts.next().unwrap().chars().nth(0).unwrap();
        let password = parts.next().unwrap().to_string();

        PasswordRecord {
            min,
            max,
            letter,
            password,
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::day2::read_password_database;

    #[test]
    fn part1() {
        let db = read_password_database();
        let valid_count = db.filter(|r| r.is_valid_part1()).count();
        println!("{}", valid_count);
    }

    #[test]
    fn part2() {
        let db = read_password_database();
        let valid_count = db.filter(|r| r.is_valid_part2()).count();
        println!("{}", valid_count);
    }
}
