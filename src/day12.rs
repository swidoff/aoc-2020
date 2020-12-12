use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day12.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self, turn: Turn, degrees: i32) -> Direction {
        if degrees == 0 {
            self.clone()
        } else {
            let next_dir = match self {
                Direction::North if turn == Turn::Right => Direction::East,
                Direction::East if turn == Turn::Right => Direction::South,
                Direction::South if turn == Turn::Right => Direction::West,
                Direction::West if turn == Turn::Right => Direction::North,
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            };

            next_dir.turn(turn, degrees - 90)
        }
    }

    fn y_amount(&self) -> i32 {
        match self {
            Direction::North => 1,
            Direction::South => -1,
            _ => 0,
        }
    }

    fn x_amount(&self) -> i32 {
        match self {
            Direction::East => 1,
            Direction::West => -1,
            _ => 0,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

impl Turn {
    fn rotate(&self, x: i32, y: i32, degrees: i32) -> (i32, i32) {
        if degrees == 0 {
            (x, y)
        } else {
            let (new_x, new_y) = match self {
                Turn::Right => (y, -x),
                _ => (-y, x),
            };

            self.rotate(new_x, new_y, degrees - 90)
        }
    }
}

enum Action {
    Move(Direction, i32),
    Turn(Turn, i32),
    Forward(i32),
}

fn parse_actions(iter: impl Iterator<Item = String>) -> impl Iterator<Item = Action> {
    iter.map(|line| {
        let (action_str, value_str) = line.split_at(1);
        let value = i32::from_str(value_str).unwrap();
        match action_str {
            "N" => Action::Move(Direction::North, value),
            "S" => Action::Move(Direction::South, value),
            "E" => Action::Move(Direction::East, value),
            "W" => Action::Move(Direction::West, value),
            "L" => Action::Turn(Turn::Left, value),
            "R" => Action::Turn(Turn::Right, value),
            "F" => Action::Forward(value),
            _ => panic!("Unknown action: {}", action_str),
        }
    })
}

fn execute_actions_part1(actions: impl Iterator<Item = Action>) -> i32 {
    let (x, y, _) = actions.fold(
        (0, 0, Direction::East),
        |(x, y, dir), action| match action {
            Action::Move(move_dir, amount) => (
                x + amount * move_dir.x_amount(),
                y + amount * move_dir.y_amount(),
                dir,
            ),
            Action::Forward(amount) => (
                x + amount * dir.x_amount(),
                y + amount * dir.y_amount(),
                dir,
            ),
            Action::Turn(turn, degrees) => (x, y, dir.turn(turn, degrees)),
        },
    );
    x.abs() + y.abs()
}

fn execute_actions_part2(actions: impl Iterator<Item = Action>) -> i32 {
    let (x, y, _, _) = actions.fold(
        (0, 0, 10, 1),
        |(ship_x, ship_y, waypoint_x, waypoint_y), action| {
            let res = match action {
                Action::Move(move_dir, amount) => (
                    ship_x,
                    ship_y,
                    waypoint_x + amount * move_dir.x_amount(),
                    waypoint_y + amount * move_dir.y_amount(),
                ),
                Action::Forward(amount) => (
                    ship_x + amount * waypoint_x,
                    ship_y + amount * waypoint_y,
                    waypoint_x,
                    waypoint_y,
                ),
                Action::Turn(turn, degrees) => {
                    let (new_waypoint_x, new_waypoint_y) =
                        turn.rotate(waypoint_x, waypoint_y, degrees);
                    (ship_x, ship_y, new_waypoint_x, new_waypoint_y)
                }
            };
            res
        },
    );
    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use crate::day12::{execute_actions_part1, execute_actions_part2, parse_actions, read_file};

    #[test]
    fn part1_example() {
        let example = "
F10
N3
F7
R90
F11
"
        .to_string();
        let actions = parse_actions(example[1..].lines().map(|s| s.to_string()));
        let res = execute_actions_part1(actions);
        assert_eq!(res, 25);
    }

    #[test]
    fn part1() {
        let iter = read_file();
        let actions = parse_actions(iter);
        let res = execute_actions_part1(actions);
        println!("{}", res);
    }

    #[test]
    fn part2_example() {
        let example = "
F10
N3
F7
R90
F11
"
        .to_string();
        let actions = parse_actions(example[1..].lines().map(|s| s.to_string()));
        let res = execute_actions_part2(actions);
        assert_eq!(res, 286);
    }

    #[test]
    fn part2() {
        let iter = read_file();
        let actions = parse_actions(iter);
        let res = execute_actions_part2(actions);
        println!("{}", res);
    }
}
