use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "grn", "gry", "hzl", "oth"];

fn read_passports() -> Vec<Passport> {
    let mut file = File::open("input/day4.txt").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    read_passports_from_string(&str)
}

fn read_passports_from_string(str: &String) -> Vec<Passport> {
    let vec = vec![Passport::new()];
    str.lines()
        .flat_map(|line| line.split(" ").into_iter())
        .fold(vec, |mut acc, input| {
            if input.is_empty() {
                acc.push(Passport::new())
            } else if let Some((key, value)) = input.split(":").collect_tuple() {
                if let Some(passport) = acc.last_mut() {
                    passport.fields.insert(key.to_string(), value.to_string());
                }
            }
            acc
        })
}

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            fields: HashMap::new(),
        }
    }

    fn is_valid(&self) -> bool {
        self.is_valid_number_of_fields() && self.all_fields_valid()
    }

    fn is_valid_number_of_fields(&self) -> bool {
        let num_fields = self.fields.len();
        num_fields == 8 || (num_fields == 7 && !self.fields.contains_key("cid"))
    }

    fn all_fields_valid(&self) -> bool {
        self.fields
            .iter()
            .all(|(field, value)| Passport::is_valid_field(field, value))
    }

    fn is_valid_field(field: &String, value: &String) -> bool {
        match field.as_str() {
            "byr" => is_valid_number(value, 1920, 2002),
            "iyr" => is_valid_number(value, 2010, 2020),
            "eyr" => is_valid_number(value, 2020, 2030),
            "hgt" if value.ends_with("cm") => is_valid_number(&value[..value.len() - 2], 150, 193),
            "hgt" if value.ends_with("in") => is_valid_number(&value[..value.len() - 2], 59, 76),
            "hcl" => value.starts_with("#") && is_valid_hex_number(&value[1..value.len()]),
            "ecl" => EYE_COLORS.binary_search(&value.as_str()).is_ok(),
            "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
            "cid" => true,
            _ => false,
        }
    }
}

fn is_valid_number(value: &str, min: u16, max: u16) -> bool {
    u16::from_str(value)
        .map(|v| v >= min && v <= max)
        .unwrap_or(false)
}

fn is_valid_hex_number(value: &str) -> bool {
    u64::from_str_radix(value, 16).is_ok()
}

#[cfg(test)]
mod tests {
    use crate::day4::{read_passports, read_passports_from_string};

    const EXAMPLE_PART1: &str = "
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";
    #[test]
    fn part1_example() {
        let passports = read_passports_from_string(&EXAMPLE_PART1[1..].to_string());
        let count = passports.iter().filter(|p| (*p).is_valid()).count();
        println!("{}", count);
        assert_eq!(count, 2);
    }

    #[test]
    fn part1() {
        let passports = read_passports();
        let count = passports
            .iter()
            .filter(|p| (*p).is_valid_number_of_fields())
            .count();
        println!("{}", count);
        assert_eq!(count, 200);
    }

    const EXAMPLE_PART2_INVALID: &str = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";

    #[test]
    fn part2_invalid() {
        let passports = read_passports_from_string(&EXAMPLE_PART2_INVALID[1..].to_string());
        let count = passports.iter().filter(|p| (*p).is_valid()).count();
        println!("{}", count);
        assert_eq!(count, 0);
    }

    const EXAMPLE_PART2_VALID: &str = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";

    #[test]
    fn part2_valid() {
        let passports = read_passports_from_string(&EXAMPLE_PART2_VALID[1..].to_string());
        let count = passports.iter().filter(|p| (*p).is_valid()).count();
        println!("{}", count);
        assert_eq!(count, 4);
    }

    #[test]
    fn part2() {
        let passports = read_passports();
        let count = passports.iter().filter(|p| (*p).is_valid()).count();
        println!("{}", count);
        assert_eq!(count, 116);
    }
}
