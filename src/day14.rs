use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day14.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

///
/// Part 1
///

enum Instruction {
    Mask {
        ones: u64,
        zeroes: u64,
        x_indexes: Vec<u8>,
    },
    Mem {
        address: u64,
        value: u64,
    },
}

fn parse_file(iter: impl Iterator<Item = String>) -> Vec<Instruction> {
    iter.map(|line| {
        let (lhs, rhs) = line.split(" = ").collect_tuple().unwrap();
        match lhs {
            "mask" => Instruction::Mask {
                ones: rhs
                    .chars()
                    .map(|ch| match ch {
                        '1' => 1,
                        _ => 0,
                    })
                    .fold(0, |acc, digit| (acc << 1) | digit),
                zeroes: rhs
                    .chars()
                    .map(|ch| match ch {
                        '0' => 0,
                        _ => 1,
                    })
                    .fold(0, |acc, digit| (acc << 1) | digit),
                x_indexes: rhs
                    .chars()
                    .enumerate()
                    .filter_map(|(i, ch)| {
                        if ch == 'X' {
                            Some((rhs.len() - i - 1) as u8)
                        } else {
                            None
                        }
                    })
                    .collect_vec(),
            },
            _ => Instruction::Mem {
                address: u64::from_str(&lhs[4..lhs.len() - 1]).unwrap(),
                value: u64::from_str(&rhs).unwrap(),
            },
        }
    })
    .collect_vec()
}

fn process_instructions(instructions: &Vec<Instruction>) -> u64 {
    let mut mem = HashMap::new();
    let mut ones_mask = u64::min_value();
    let mut zeroes_mask = u64::max_value();

    for instruction in instructions {
        match instruction {
            Instruction::Mask {
                ones,
                zeroes,
                x_indexes,
            } => {
                ones_mask = *ones;
                zeroes_mask = *zeroes;
            }
            Instruction::Mem { address, value } => {
                let masked_value = (value | ones_mask) & zeroes_mask;
                mem.insert(address, masked_value);
            }
        };
    }

    mem.values().sum()
}

///
/// Part 2
///
fn process_instructions_v2(instructions: &Vec<Instruction>) -> u64 {
    let mut mem = HashMap::new();
    let mut ones_mask = u64::min_value();
    let mut x_indexes_mask = None;

    for instruction in instructions {
        match instruction {
            Instruction::Mask {
                ones,
                zeroes: _zeroes,
                x_indexes,
            } => {
                ones_mask = *ones;
                x_indexes_mask.replace(x_indexes);
            }
            Instruction::Mem { address, value } => {
                let base_address = (address | ones_mask);
                for final_address in enumerate_addresses(base_address, x_indexes_mask.unwrap()) {
                    mem.insert(final_address, *value);
                }
            }
        };
    }

    mem.values().sum()
}

fn enumerate_addresses(base_address: u64, x_indexes: &Vec<u8>) -> VecDeque<u64> {
    let capacity: usize = (2 as usize).pow(x_indexes.len() as u32) as usize;
    let mut res = VecDeque::with_capacity(capacity);
    res.push_back(base_address);

    for x_index in x_indexes {
        let ones_mask = 1 << x_index;
        let zeroes_mask = !ones_mask;

        for _ in 0..res.len() {
            let addr = res.pop_front().unwrap();
            res.push_back(addr & zeroes_mask);
            res.push_back(addr | ones_mask);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::day14::{parse_file, process_instructions, process_instructions_v2, read_file};

    #[test]
    fn test_part1_example() {
        let example = "
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";
        let instructions = parse_file(example[1..].lines().map(|s| s.to_string()));
        let res = process_instructions(&instructions);
        assert_eq!(res, 165);
    }

    #[test]
    fn part1() {
        let instructions = parse_file(read_file());
        let res = process_instructions(&instructions);
        println!("{}", res);
        assert_eq!(res, 4886706177792);
    }

    #[test]
    fn test_part2_example() {
        let example = "
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";
        let instructions = parse_file(example[1..].lines().map(|s| s.to_string()));
        let res = process_instructions_v2(&instructions);
        assert_eq!(res, 208);
    }

    #[test]
    fn part2() {
        let instructions = parse_file(read_file());
        let res = process_instructions_v2(&instructions);
        println!("{}", res);
        assert_eq!(res, 3348493585827);
    }
}
