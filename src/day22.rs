use itertools::Itertools;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day22.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

fn parse_decks(iter: impl Iterator<Item = String>) -> (VecDeque<u64>, VecDeque<u64>) {
    let mut deck1 = VecDeque::new();
    let mut deck2 = VecDeque::new();

    let mut switched = false;
    for line in iter.filter(|line| !line.starts_with("Player")) {
        if line.is_empty() {
            switched = true;
        } else if switched {
            deck2.push_back(u64::from_str(&line).unwrap());
        } else {
            deck1.push_back(u64::from_str(&line).unwrap())
        }
    }

    (deck1, deck2)
}

fn combat<'a>(deck1: &'a mut VecDeque<u64>, deck2: &'a mut VecDeque<u64>) -> (usize, u64) {
    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    calc_result(&deck1, &deck2)
}

fn calc_result(deck1: &VecDeque<u64>, deck2: &VecDeque<u64>) -> (usize, u64) {
    let (winner, winning_deck) = if deck1.is_empty() {
        (2, &deck2)
    } else {
        (1, &deck1)
    };

    let winning_score = winning_deck
        .iter()
        .zip((1..winning_deck.len() + 1).rev())
        .map(|(card, pos)| *card * pos as u64)
        .sum();

    (winner, winning_score)
}

fn recursive_combat<'a>(
    deck1: &'a mut VecDeque<u64>,
    deck2: &'a mut VecDeque<u64>,
) -> (usize, u64) {
    let mut states = HashSet::new();

    while !deck1.is_empty() && !deck2.is_empty() {
        let state = (deck1.clone(), deck2.clone());
        if states.contains(&state) {
            deck2.clear();
        } else {
            let card1 = deck1.pop_front().unwrap();
            let card2 = deck2.pop_front().unwrap();

            let winner = if card1 <= (deck1.len() as u64) && card2 <= (deck2.len() as u64) {
                let mut sub_deck1 =
                    VecDeque::from_iter(deck1.iter().take(card1 as usize).map(|v| *v));
                let mut sub_deck2 =
                    VecDeque::from_iter(deck2.iter().take(card2 as usize).map(|v| *v));
                let (sub_game_winner, _) = recursive_combat(&mut sub_deck1, &mut sub_deck2);
                sub_game_winner
            } else if card1 > card2 {
                1
            } else {
                2
            };

            if winner == 1 {
                deck1.push_back(card1);
                deck1.push_back(card2);
            } else {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }

            states.insert(state);
        }
    }

    calc_result(&deck1, &deck2)
}

mod tests {
    use crate::day22::{combat, parse_decks, read_file, recursive_combat};

    const EXAMPLE: &str = "
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

    #[test]
    fn test_part1_example() {
        let iter = EXAMPLE[1..].lines().map(|line| line.to_string());
        let (mut deck1, mut deck2) = parse_decks(iter);
        let (_, res) = combat(&mut deck1, &mut deck2);
        assert_eq!(306, res);
    }

    #[test]
    fn test_part1() {
        let iter = read_file();
        let (mut deck1, mut deck2) = parse_decks(iter);
        let (_, res) = combat(&mut deck1, &mut deck2);
        // println!("{}", res);
        assert_eq!(30138, res);
    }

    #[test]
    fn test_part2_example1() {
        let iter = EXAMPLE[1..].lines().map(|line| line.to_string());
        let (mut deck1, mut deck2) = parse_decks(iter);
        let (_, res) = recursive_combat(&mut deck1, &mut deck2);
        assert_eq!(291, res);
    }

    #[test]
    fn test_part2_example2() {
        let example = "Player 1:
43
19

Player 2:
2
29
14
";
        let iter = example.lines().map(|line| line.to_string());
        let (mut deck1, mut deck2) = parse_decks(iter);
        let (winner, _) = recursive_combat(&mut deck1, &mut deck2);
        assert_eq!(1, winner);
        // assert_eq!(291, res);
    }

    #[test]
    fn test_part2() {
        let iter = read_file();
        let (mut deck1, mut deck2) = parse_decks(iter);
        let (_, res) = recursive_combat(&mut deck1, &mut deck2);
        // println!("{}", res);
        assert_eq!(31587, res);
    }
}
