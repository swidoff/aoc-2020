use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day13.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

///
/// Part 1
///

fn parse_file_part1(lines: impl Iterator<Item = String>) -> (u32, Vec<u32>) {
    let (line1, line2) = lines.take(2).collect_tuple().unwrap();
    let timestamp = u32::from_str(line1.as_str()).unwrap();
    let bus_numbers = line2
        .split(",")
        .filter(|n| *n != "x")
        .map(|n| u32::from_str(n).unwrap())
        .collect_vec();
    (timestamp, bus_numbers)
}

fn find_earliest_bus_and_wait_time(timestamp: u32, bus_numbers: &Vec<u32>) -> (u32, u32) {
    bus_numbers
        .iter()
        .map(|bus_number| {
            let arrival_time = (timestamp / bus_number) * bus_number
                + (if timestamp % bus_number != 0 {
                    *bus_number
                } else {
                    0
                });
            let wait_time = arrival_time - timestamp;
            (*bus_number, wait_time)
        })
        .min_by_key(|(_bus_number, wait_time)| *wait_time)
        .unwrap()
}

///
/// Part 2
///

fn parse_file_part2(lines: impl Iterator<Item = String>) -> Vec<(u64, u64)> {
    let line = lines.skip(1).next().unwrap();
    parse_line_part2(&line)
}

/// Returns vector of tuples of first departure time and bus number
fn parse_line_part2(line: &String) -> Vec<(u64, u64)> {
    line.split(",")
        .enumerate()
        .filter(|(_, n)| *n != "x")
        .map(|(i, n)| (i as u64, u64::from_str(n).unwrap()))
        .collect_vec()
}

/// Merge each bus with the next so the new bus has the combined departure time and bus number (period).
/// You can merge when the buses arrive with the correct offset twice.
/// The first time they meet is the new departure; the second time is the new bus number;
fn solve_contest(inputs: &Vec<(u64, u64)>) -> u64 {
    let mut departure_time = 0;
    let mut bus_number = inputs[0].1;
    for (i, (next_departure_time, next_bus_number)) in inputs.iter().enumerate().skip(1) {
        let mut t = departure_time;
        let mut new_departure_time = None;
        let mut new_bus_number = 0;

        while new_bus_number == 0 {
            t += bus_number;

            if (t + next_departure_time) % next_bus_number == 0 {
                if i == inputs.len() - 1 {
                    new_bus_number = t;
                } else if let Some(dep_time) = new_departure_time {
                    new_bus_number = t - dep_time;
                    departure_time = dep_time;
                } else {
                    new_departure_time = Some(t);
                }
            }
        }

        bus_number = new_bus_number;
    }
    bus_number
}

#[cfg(test)]
mod tests {
    use crate::day13::{
        find_earliest_bus_and_wait_time, parse_file_part1, parse_file_part2, parse_line_part2,
        read_file, solve_contest,
    };

    #[test]
    fn test_part1_example() {
        let example = "
939
7,13,x,x,59,x,31,19
";
        let (timestamp, bus_numbers) =
            parse_file_part1(example[1..].lines().map(|s| s.to_string()));
        let (bus_number, wait_time) = find_earliest_bus_and_wait_time(timestamp, &bus_numbers);
        assert_eq!(295, bus_number * wait_time);
    }

    #[test]
    fn test_part1() {
        let (timestamp, bus_numbers) = parse_file_part1(read_file());
        let (bus_number, wait_time) = find_earliest_bus_and_wait_time(timestamp, &bus_numbers);
        println!("{}", bus_number * wait_time);
        assert_eq!(3035, bus_number * wait_time);
    }

    #[test]
    fn test_part2_example() {
        let line = "17,x,13,19";
        let input = parse_line_part2(&line.to_string());
        let res = solve_contest(&input);
        assert_eq!(3417, res);

        let line = "67,7,59,61";
        let input = parse_line_part2(&line.to_string());
        let res = solve_contest(&input);
        assert_eq!(754018, res);

        let line = "67,x,7,59,61";
        let input = parse_line_part2(&line.to_string());
        let res = solve_contest(&input);
        assert_eq!(779210, res);

        let line = "67,7,x,59,61";
        let input = parse_line_part2(&line.to_string());
        let res = solve_contest(&input);
        assert_eq!(1261476, res);

        let line = "1789,37,47,1889";
        let input = parse_line_part2(&line.to_string());
        let res = solve_contest(&input);
        assert_eq!(1202161486, res);

        let line = "7,13,x,x,59,x,31,19";
        let input = parse_line_part2(&line.to_string());
        let res = solve_contest(&input);
        assert_eq!(1068781, res);
    }

    #[test]
    fn test_part2() {
        let input = parse_file_part2(read_file());
        let res = solve_contest(&input);
        println!("{}", res);
        assert_eq!(725169163285238, res);
    }
}
