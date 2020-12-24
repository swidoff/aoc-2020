use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

fn parse_input(input: &String) -> Vec<usize> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec()
}

/// Part 1

fn play_cups(input: &Vec<usize>, moves: usize) -> String {
    let mut cups = Vec::from_iter(input.iter().map(|d| *d));
    let mut current_index = 0;
    let mut q = VecDeque::with_capacity(3);

    for _ in 0..moves {
        let current_value = cups[current_index];
        for _ in 1..4 {
            if current_index + 1 < cups.len() {
                q.push_back(cups.remove(current_index + 1));
            } else {
                q.push_back(cups.remove(0))
            }
        }

        let mut dest_value = current_value - 1;
        while !cups.contains(&dest_value) {
            if dest_value == 0 {
                dest_value = input.len() as usize;
            } else {
                dest_value -= 1
            }
        }

        if let Some((dest_index, _)) = cups.iter().find_position(|d| **d == dest_value) {
            while let Some(v) = q.pop_back() {
                cups.insert(dest_index + 1, v);
            }
        }

        if let Some((new_current_index, _)) = cups.iter().find_position(|d| **d == current_value) {
            current_index = (new_current_index + 1) % cups.len();
        }
    }

    let mut res = String::new();
    if let Some((index, _)) = cups.iter().find_position(|d| **d == 1) {
        for i in 1..cups.len() {
            let v = cups[(index + i) % cups.len()];
            res.push_str(&v.to_string());
        }
    }

    res
}

/// Part 2

struct NodeMap {
    vec: Vec<usize>,
}

impl NodeMap {
    fn new(input: &Vec<usize>) -> NodeMap {
        let mut vec = vec![0; input.len() + 1];

        for (i, v) in input.iter().enumerate() {
            let next = input[(i + 1) % input.len()];
            vec[*v] = next;
        }

        NodeMap { vec }
    }

    fn get_next(&self, v: usize) -> usize {
        self.vec[v]
    }

    fn contains(&self, v: usize) -> bool {
        self.vec[v] != 0
    }

    fn remove_next(&mut self, v: usize) -> usize {
        let next_value = self.vec[v];
        let new_next_value = self.vec[next_value];
        self.vec[next_value] = 0;
        self.vec[v] = new_next_value;
        next_value
    }

    fn insert_next(&mut self, at: usize, v: usize) {
        let next_value = self.vec[at];
        self.vec[v] = next_value;
        self.vec[at] = v;
    }
}

fn play_cups_faster(input: &Vec<usize>, moves: usize) -> usize {
    let mut node_map = NodeMap::new(input);

    let mut current_value = input[0];
    let mut q = VecDeque::with_capacity(3);
    for _ in 0..moves {
        for _ in 1..4 {
            q.push_back(node_map.remove_next(current_value));
        }

        let mut dest_value = current_value - 1;
        while !node_map.contains(dest_value) {
            if dest_value == 0 {
                dest_value = input.len() as usize;
            } else {
                dest_value -= 1
            }
        }

        while let Some(v) = q.pop_back() {
            node_map.insert_next(dest_value, v);
        }

        current_value = node_map.get_next(current_value);
    }

    let v1 = node_map.get_next(1);
    let v2 = node_map.get_next(v1);
    v1 * v2
}

fn add_cups(cups: &mut Vec<usize>, n: usize) {
    for i in (cups.len() as usize)..n {
        cups.push(i + 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::day23::{add_cups, parse_input, play_cups, play_cups_faster};

    #[test]
    fn test_part1_example() {
        let input = parse_input(&"389125467".to_string());
        let res = play_cups(&input, 10);
        assert_eq!(res, "92658374");

        let res = play_cups(&input, 100);
        assert_eq!(res, "67384529");
    }

    #[test]
    fn test_part1() {
        let input = parse_input(&"685974213".to_string());
        let res = play_cups(&input, 100);
        println!("{}", res);
        assert_eq!(res, "82635947");
    }

    #[test]
    fn test_part2_example1() {
        let input = parse_input(&"389125467".to_string());
        let res = play_cups_faster(&input, 10);
        assert_eq!(res, 18);

        let res = play_cups_faster(&input, 100);
        assert_eq!(res, 42);
    }

    #[test]
    fn test_part2_example2() {
        let mut input = parse_input(&"389125467".to_string());
        add_cups(&mut input, 1000000);
        let res = play_cups_faster(&input, 10000000);
        assert_eq!(res, 149245887792);
    }

    #[test]
    fn test_part2() {
        let mut input = parse_input(&"685974213".to_string());
        add_cups(&mut input, 1000000);
        let res = play_cups_faster(&input, 10000000);
        println!("{}", res);
        assert_eq!(res, 157047826689);
    }
}
