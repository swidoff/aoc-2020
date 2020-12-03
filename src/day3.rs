use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

fn read_grid() -> Vec<Vec<char>> {
    let mut file = File::open("input/day3.txt").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    read_grid_from_string(&str)
}

fn read_grid_from_string(str: &String) -> Vec<Vec<char>> {
    str.lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        println!("{}", String::from_iter(line.iter()));
    }
}

fn count_trees_on_path(grid: &Vec<Vec<char>>, x_velocity: usize, y_velocity: usize) -> u64 {
    let (count, _) = grid
        .iter()
        .step_by(y_velocity)
        .fold((0, 0), |(count, pos_x), row| {
            let new_count = count + (if row[pos_x] == '#' { 1 } else { 0 });
            let new_pos_x = (pos_x + x_velocity) % row.len();
            (new_count, new_pos_x)
        });
    count
}

#[cfg(test)]
mod tests {
    use crate::day3::{count_trees_on_path, read_grid, read_grid_from_string};

    const EXAMPLE_GRID: &str = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

    #[test]
    fn part1_example() {
        let grid = read_grid_from_string(&EXAMPLE_GRID[1..].to_string());
        let count = count_trees_on_path(&grid, 3, 1);
        println!("{}", count);
    }

    #[test]
    fn part1() {
        let grid = read_grid();
        let count = count_trees_on_path(&grid, 3, 1);
        println!("{}", count);
    }

    #[test]
    fn part2_example() {
        let grid = read_grid_from_string(&EXAMPLE_GRID[1..].to_string());
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let res: u64 = slopes.iter().fold(1, |res, (x_velocity, y_velocity)| {
            let count = count_trees_on_path(&grid, *x_velocity, *y_velocity);
            println!("{}", count);
            res * count
        });
        println!("{}", res);
    }

    #[test]
    fn part2() {
        let grid = read_grid();
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let res: u64 = slopes.iter().fold(1, |res, (x_velocity, y_velocity)| {
            res * count_trees_on_path(&grid, *x_velocity, *y_velocity)
        });
        println!("{}", res);
    }
}
