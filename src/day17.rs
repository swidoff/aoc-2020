use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day17.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

type Coord = (i64, i64, i64, i64);

fn initial_state(iter: impl Iterator<Item = String>, has_w: bool) -> HashMap<Coord, bool> {
    let mut state = HashMap::new();
    for (y, line) in iter.enumerate() {
        for (x, char) in line.chars().enumerate() {
            let active = char == '#';
            let coords = (x as i64, y as i64, 0, 0);
            state.insert(coords, active);

            if active {
                for c in neighbors(&coords, has_w) {
                    if !state.contains_key(&c) {
                        state.insert(c, false);
                    }
                }
            }
        }
    }
    state
}

fn neighbors(coords: &Coord, has_w: bool) -> impl Iterator<Item = Coord> {
    let (x, y, z, w) = *coords;
    let (w_min, w_max) = if has_w { (-1, 2) } else { (0, 1) };
    (-1..2)
        .flat_map(move |xd| {
            (-1..2).flat_map(move |yd| {
                (-1..2).flat_map(move |zd| (w_min..w_max).map(move |wd| (xd, yd, zd, wd)))
            })
        })
        .filter(|(xd, yd, zd, wd)| !(*xd == 0 && *yd == 0 && *zd == 0 && *wd == 0))
        .map(move |(xd, yd, zd, wd)| (x + xd, y + yd, z + zd, w + wd))
}

fn boot_process(state: &mut HashMap<Coord, bool>, cycles: usize, has_w: bool) {
    let mut changes: HashMap<Coord, bool> = HashMap::new();
    for _ in 0..cycles {
        for (coords, active) in state.iter() {
            let active_neighbors: i64 = neighbors(coords, has_w)
                .map(|c| {
                    state
                        .get(&c)
                        .map(|active| if *active { 1 } else { 0 })
                        .unwrap_or(0)
                })
                .sum();

            if *active && active_neighbors != 2 && active_neighbors != 3 {
                changes.insert(*coords, false);
            } else if !*active && active_neighbors == 3 {
                changes.insert(*coords, true);
            }
        }

        for (coords, active) in changes.iter() {
            state.insert(*coords, *active);

            if *active {
                for c in neighbors(coords, has_w) {
                    if !state.contains_key(&c) {
                        state.insert(c, false);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day17::{boot_process, initial_state, read_file};

    #[test]
    fn test_part1_example() {
        let example = "
.#.
..#
###
";
        let mut state = initial_state(example[1..].lines().map(|line| line.to_string()), false);
        boot_process(&mut state, 6, false);
        let res = state.values().filter(|active| **active).count();
        // println!("{}", res);
        assert_eq!(res, 112);
    }

    #[test]
    fn test_part1() {
        let mut state = initial_state(read_file(), false);
        boot_process(&mut state, 6, false);
        let res = state.values().filter(|active| **active).count();
        println!("{}", res);
        assert_eq!(res, 322);
    }

    #[test]
    fn test_part2_example() {
        let example = "
.#.
..#
###
";
        let mut state = initial_state(example[1..].lines().map(|line| line.to_string()), true);
        boot_process(&mut state, 6, true);
        let res = state.values().filter(|active| **active).count();
        // println!("{}", res);
        assert_eq!(res, 848);
    }

    #[test]
    fn test_part2() {
        let mut state = initial_state(read_file(), true);
        boot_process(&mut state, 6, true);
        let res = state.values().filter(|active| **active).count();
        // println!("{}", res);
        assert_eq!(res, 2000);
    }
}
