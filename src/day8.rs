use crate::day8::Instruction::ACC;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day8.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

#[derive(Debug)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

fn parse_instructions(iter: impl Iterator<Item = String>) -> Vec<Instruction> {
    iter.map(|line| {
        let sign = match &line[4..5] {
            "+" => 1,
            _ => -1,
        };
        let value = i32::from_str(&line[5..]).unwrap();
        let argument = sign * value;

        match &line[0..3] {
            "acc" => Instruction::ACC(argument),
            "jmp" => Instruction::JMP(argument),
            "nop" => Instruction::NOP(argument),
            x => panic!("Unexpected argument: {}", x),
        }
    })
    .collect_vec()
}

fn run_code(program: &Vec<Instruction>) -> Result<i32, i32> {
    let mut seen = HashSet::new();
    let mut i = 0;
    let mut acc = 0;
    let len = program.len() as i32;
    while i < len {
        if seen.contains(&i) {
            return Result::Err(acc);
        } else {
            seen.insert(i);
        }

        match &program[i as usize] {
            Instruction::ACC(argument) => {
                acc += argument;
                i += 1;
            }
            Instruction::JMP(argument) => {
                i += argument;
            }
            Instruction::NOP(_) => {
                i += 1;
            }
        }
    }

    Result::Ok(acc)
}

fn break_infinite_loop(program: &mut Vec<Instruction>) -> Option<i32> {
    let candidates = program
        .iter()
        .enumerate()
        .filter_map(|(index, instruction)| match instruction {
            Instruction::ACC(_) => None,
            _ => Some(index),
        })
        .collect_vec();

    candidates.iter().find_map(|index| {
        let replacement = match &program[*index] {
            Instruction::NOP(arg) => Instruction::JMP(*arg),
            Instruction::JMP(arg) => Instruction::NOP(*arg),
            _ => panic!("Unexpected instruction"),
        };

        // Replace candidate
        program.push(replacement);
        let old_instruction = program.swap_remove(*index);

        match run_code(&program) {
            Ok(acc) => Some(acc),
            Err(_) => {
                // Restore candidate.
                program.push(old_instruction);
                program.swap_remove(*index);
                None
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::day8::{break_infinite_loop, parse_instructions, read_file, run_code};
    use std::collections::HashSet;

    const EXAMPLE1: &str = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

    #[test]
    fn part1_example() {
        let instructions =
            parse_instructions(EXAMPLE1[1..].to_string().lines().map(|s| s.to_string()));
        let res = run_code(&instructions);
        assert_eq!(res, Err(5));
    }

    #[test]
    fn part1() {
        let instructions = parse_instructions(read_file());
        let res = run_code(&instructions);
        assert_eq!(res, Err(1331));
    }

    #[test]
    fn part2_example() {
        let mut instructions =
            parse_instructions(EXAMPLE1[1..].to_string().lines().map(|s| s.to_string()));
        let res = break_infinite_loop(&mut instructions);
        assert_eq!(res, Some(8));
    }

    #[test]
    fn part2() {
        let mut instructions = parse_instructions(read_file());
        let res = break_infinite_loop(&mut instructions);
        println!("{}", res.unwrap());
    }
}
