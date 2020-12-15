use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split(",")
        .map(|s| u64::from_str(s).unwrap())
        .collect_vec()
}

struct MemoryGame {
    record: HashMap<u64, VecDeque<u64>>,
    turn: u64,
    last_spoken: u64,
}

impl MemoryGame {
    fn new() -> MemoryGame {
        MemoryGame {
            record: HashMap::new(),
            turn: 1,
            last_spoken: 0,
        }
    }

    fn speak(&mut self, value: u64) {
        match self.record.get_mut(&value) {
            Some(deque) => deque.push_front(self.turn),
            None => {
                let mut deque = VecDeque::with_capacity(2);
                deque.push_back(self.turn);
                self.record.insert(value, deque);
            }
        }

        self.turn += 1;
        self.last_spoken = value;
    }

    fn next(&self) -> u64 {
        match self.record.get(&self.last_spoken) {
            Some(deque) if deque.len() > 1 => deque[0] - deque[1],
            _ => 0,
        }
    }
}

fn memory_game(starting_numbers: &Vec<u64>, iterations: u64) -> u64 {
    let mut game = MemoryGame::new();
    for n in starting_numbers {
        game.speak(*n);
    }

    while game.turn < iterations {
        let next = game.next();
        game.speak(next);
    }

    game.next()
}

#[cfg(test)]
mod tests {
    use crate::day15::{memory_game, parse_input};

    #[test]
    fn test_part1_example() {
        let example = "0,3,6";
        let inputs = parse_input(example);
        assert_eq!(memory_game(&inputs, 2020), 436);
    }

    #[test]
    fn test_part1() {
        let example = "12,1,16,3,11,0";
        let inputs = parse_input(example);
        let res = memory_game(&inputs, 2020);
        println!("{}", res);
        assert_eq!(memory_game(&inputs, 2020), 1696);
    }

    #[test]
    fn test_part2_example() {
        let example = "0,3,6";
        let inputs = parse_input(example);
        assert_eq!(memory_game(&inputs, 2020), 436);
    }

    #[test]
    fn test_part2() {
        let example = "12,1,16,3,11,0";
        let inputs = parse_input(example);
        let res = memory_game(&inputs, 30000000);
        println!("{}", res);
        assert_eq!(res, 37385);
    }
}
