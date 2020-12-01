use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;

/// Assuming file does not contain duplicates (mine does not)
/// Assuming the sum does not pair (or triple) a number with itself (1010 is not in the file)

fn read_file_as_map() -> HashMap<u32, u32> {
    let file = File::open("input/day1.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();

    let mut res = HashMap::new();
    for line in string.lines() {
        let v = u32::from_str(line).unwrap();
        if res.insert(v, v, ).is_some() {
            panic!("File contains a duplicate value: {}", v);
        }
    }
    res
}

fn find_pair_with_sum(input: &HashMap<u32, u32>, sum: u32) -> Option<(u32, u32)> {
    let mut res = None;
    for v1 in input.keys() {
        if *v1 < sum {
            let v2 = sum - *v1;
            if input.contains_key(&v2) {
                if *v1 != v2 {
                    res = Some((*v1, v2))
                } else {
                    panic!("Solution involved matching a number with itself")
                }
            }
        }
    }
    res
}

fn find_triple_with_sum(input: &HashMap<u32, u32>, sum: u32) -> Option<(u32, u32, u32)> {
    let mut res = None;
    for v1 in input.keys() {
        if *v1 < sum {
            let pair_sum = sum - *v1;
            if let Some((v2, v3)) = find_pair_with_sum(input, pair_sum) {
                if *v1 != v2 && *v1 != v3 {
                    res = Some((*v1, v2, v3))
                } else {
                    panic!("Solution involved matching a number with itself")
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::day1::{find_pair_with_sum, find_triple_with_sum, read_file_as_map};

    #[test]
    fn part1() {
        let input = read_file_as_map();
        match find_pair_with_sum(&input, 2020) {
            Some((v1, v2)) => println!("{}", v1 * v2),
            None => println!("No solution found")
        }
    }

    #[test]
    fn part2() {
        let input = read_file_as_map();
        match find_triple_with_sum(&input, 2020) {
            Some((v1, v2, v3)) => println!("{}", v1 * v2 * v3),
            None => println!("No solution found")
        }
    }
}
