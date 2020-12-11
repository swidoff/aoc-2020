use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day11.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

fn parse_layout(iter: impl Iterator<Item = String>) -> HashMap<(i32, i32), Position> {
    let mut res = HashMap::new();
    for (x, line) in iter.enumerate() {
        for (y, ch) in line.chars().enumerate() {
            let position = match ch {
                '.' => Position::Floor,
                'L' => Position::Empty,
                '#' => Position::Occupied,
                pos => panic!("Illegal starting position char: {}", pos),
            };
            res.insert((x as i32, y as i32), position);
        }
    }
    res
}

//-------
// Part 1
//-------

/// Counts the occupied chairs in any of the eight adjacent spaces.
fn count_adjacent_occupied(x: i32, y: i32, layout: &HashMap<(i32, i32), Position>) -> u32 {
    let mut count = 0;
    for xv in x - 1..x + 2 {
        for yv in y - 1..y + 2 {
            if !(xv == x && yv == y) {
                if let Some(Position::Occupied) = layout.get(&(xv, yv)) {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn model_waiting_room_part1(layout: &mut HashMap<(i32, i32), Position>) -> usize {
    let mut changes: HashMap<(i32, i32), Position> = HashMap::new();

    let mut changed: usize = usize::max_value();
    while changed > 0 {
        for (key, pos) in layout.iter() {
            match pos {
                Position::Empty if count_adjacent_occupied(key.0, key.1, &layout) == 0 => {
                    changes.insert(*key, Position::Occupied);
                }
                Position::Occupied if count_adjacent_occupied(key.0, key.1, &layout) >= 4 => {
                    changes.insert(*key, Position::Empty);
                }
                _ => {}
            };
        }

        for (key, pos) in changes.iter() {
            layout.insert(*key, *pos);
        }

        changed = changes.len();
        changes.clear();
    }

    layout
        .values()
        .filter(|p| **p == Position::Occupied)
        .count()
}

//-------
// Part 2
//-------

/// Counts the first occupied chair in any of the cardinal directions. An unoccupied chair obstructs the view further.
fn count_first_occupied(x: i32, y: i32, layout: &HashMap<(i32, i32), Position>) -> u32 {
    let mut count = 0;
    for xd in [-1, 0, 1].iter() {
        for yd in [-1, 0, 1].iter() {
            if *xd == 0 && *yd == 0 {
                continue;
            }

            let mut xv = x + xd;
            let mut yv = y + yd;
            loop {
                // Loop will exit. We'll either hit a chair or leave the layout (empty in the HashMap).
                let res = layout.get(&(xv, yv));
                if let Some(Position::Floor) = res {
                    xv += xd;
                    yv += yd;
                } else {
                    if let Some(Position::Occupied) = res {
                        count += 1;
                    }
                    break;
                }
            }
        }
    }
    return count;
}

fn model_waiting_room_part2(layout: &mut HashMap<(i32, i32), Position>) -> usize {
    let mut changes: HashMap<(i32, i32), Position> = HashMap::new();

    let mut changed: usize = usize::max_value();
    while changed > 0 {
        for (key, pos) in layout.iter() {
            match pos {
                Position::Empty if count_first_occupied(key.0, key.1, &layout) == 0 => {
                    changes.insert(*key, Position::Occupied);
                }
                Position::Occupied if count_first_occupied(key.0, key.1, &layout) >= 5 => {
                    changes.insert(*key, Position::Empty);
                }
                _ => {}
            };
        }

        for (key, pos) in changes.iter() {
            layout.insert(*key, *pos);
        }

        changed = changes.len();
        changes.clear();
    }

    layout
        .values()
        .filter(|p| **p == Position::Occupied)
        .count()
}

// fn print_layout(layout: &HashMap<(i32, i32), Position>, dim_x: usize, dim_y: usize) {
//     for x in 0..dim_x {
//         for y in 0..dim_y {
//             let ch = match layout.get(&(x as i32, y as i32)) {
//                 Some(Position::Empty) => 'L',
//                 Some(Position::Occupied) => '#',
//                 Some(Position::Floor) => '.',
//                 _ => panic!("Unknown position"),
//             };
//             print!("{}", ch);
//         }
//         println!();
//     }
// }

#[cfg(test)]
mod tests {
    use crate::day11::{
        count_first_occupied, model_waiting_room_part1, model_waiting_room_part2, parse_layout,
        read_file,
    };

    const EXAMPLE: &str = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    #[test]
    fn part1_example() {
        let mut layout = parse_layout(EXAMPLE[1..].to_string().lines().map(|s| s.to_string()));
        let res = model_waiting_room_part1(&mut layout);
        assert_eq!(res, 37);
    }

    #[test]
    fn part1() {
        let mut layout = parse_layout(read_file());
        let res = model_waiting_room_part1(&mut layout);
        assert_eq!(2265, res);
    }

    #[test]
    fn part2_example() {
        let test1 = "
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....
";
        let layout = parse_layout(test1[1..].to_string().lines().map(|s| s.to_string()));
        // print_layout(&layout, 9, 9);
        assert_eq!(8, count_first_occupied(4, 3, &layout));

        let test2 = "
.............
.L.L.#.#.#.#.
.............
";
        let layout = parse_layout(test2[1..].to_string().lines().map(|s| s.to_string()));
        // print_layout(&layout, 3, 13);
        assert_eq!(0, count_first_occupied(1, 1, &layout));

        let test3 = "
.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.
";
        let layout = parse_layout(test3[1..].to_string().lines().map(|s| s.to_string()));
        // print_layout(&layout, 3, 13);
        assert_eq!(0, count_first_occupied(3, 3, &layout));

        let mut layout = parse_layout(EXAMPLE[1..].to_string().lines().map(|s| s.to_string()));
        let res = model_waiting_room_part2(&mut layout);
        assert_eq!(res, 26);
    }

    #[test]
    fn part2() {
        let mut layout = parse_layout(read_file());
        let res = model_waiting_room_part2(&mut layout);
        assert_eq!(2045, res);
    }
}
