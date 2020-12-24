use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day24.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

type Coord = (i64, i64);

fn flip_tiles(iter: impl Iterator<Item = String>) -> HashMap<Coord, u64> {
    let mut tiles: HashMap<Coord, u64> = HashMap::new();
    for line in iter {
        let coord = coord_for_line(line);
        if let Some(v) = tiles.get(&coord) {
            let v = *v;
            tiles.insert(coord, (v + 1) % 2);
        } else {
            tiles.insert(coord, 1);
        }
    }
    tiles
}

fn part1(iter: impl Iterator<Item = String>) -> u64 {
    let tiles = flip_tiles(iter);
    tiles.values().sum()
}

fn coord_for_line(line: String) -> Coord {
    let mut ref_x: i64 = 0;
    let mut ref_y: i64 = 0;
    let mut chars = line.chars();
    while let Some(c1) = chars.next() {
        match c1 {
            'e' => ref_x += 2,
            'w' => ref_x -= 2,
            's' => match chars.next().unwrap() {
                'e' => {
                    ref_x += 1;
                    ref_y += 1;
                }
                'w' => {
                    ref_x -= 1;
                    ref_y += 1;
                }
                _ => {}
            },
            'n' => match chars.next().unwrap() {
                'e' => {
                    ref_x += 1;
                    ref_y -= 1;
                }
                'w' => {
                    ref_x -= 1;
                    ref_y -= 1;
                }
                _ => {}
            },
            _ => {}
        }
    }
    (ref_x, ref_y)
}

const ADJ_TILES: [Coord; 6] = [(-2, 0), (2, 0), (-1, -1), (-1, 1), (1, 1), (1, -1)];

/// Adds any missing white tiles around a black tile.
fn expand(tiles: &mut HashMap<Coord, u64>) {
    let mut changes = HashMap::new();
    for ((x, y), v) in tiles.iter() {
        if *v == 1 {
            for (xd, yd) in ADJ_TILES.iter() {
                let coord = (*x + *xd, *y + *yd);
                if !tiles.contains_key(&coord) {
                    changes.insert(coord, 0);
                }
            }
        }
    }
    for (c, v) in changes.iter() {
        tiles.insert(*c, *v);
    }
}

fn part2(iter: impl Iterator<Item = String>, turns: usize) -> u64 {
    let mut tiles = flip_tiles(iter);
    let mut changes = HashMap::new();

    for _ in 0..turns {
        expand(&mut tiles);

        for (&coord, v) in tiles.iter() {
            let (x, y) = coord;
            let adj_black: u64 = ADJ_TILES
                .iter()
                .map(|&(xd, yd)| tiles.get(&(x + xd, y + yd)).map(|v| *v).unwrap_or(0))
                .sum();

            if *v == 1 && (adj_black == 0 || adj_black > 2) {
                changes.insert(coord, 0);
            } else if *v == 0 && adj_black == 2 {
                changes.insert(coord, 1);
            }
        }

        for (c, v) in changes.iter() {
            tiles.insert(*c, *v);
        }

        changes.clear();
    }
    tiles.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::day24::{coord_for_line, part1, part2, read_file};

    const EXAMPLE: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwsesweswv
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";

    #[test]
    fn test_part1_example() {
        assert_eq!((0, 0), coord_for_line("nwwswee".to_string()));

        let res = part1(EXAMPLE.lines().map(|l| l.to_string()));
        assert_eq!(res, 10);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(394, res);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|l| l.to_string()), 1), 15);
        assert_eq!(part2(EXAMPLE.lines().map(|l| l.to_string()), 2), 12);
        assert_eq!(part2(EXAMPLE.lines().map(|l| l.to_string()), 3), 25);
        assert_eq!(part2(EXAMPLE.lines().map(|l| l.to_string()), 4), 14);
        assert_eq!(part2(EXAMPLE.lines().map(|l| l.to_string()), 5), 23);
        assert_eq!(part2(EXAMPLE.lines().map(|l| l.to_string()), 6), 28);
        assert_eq!(part2(EXAMPLE.lines().map(|l| l.to_string()), 7), 41);
        assert_eq!(part2(EXAMPLE.lines().map(|l| l.to_string()), 8), 37);
        assert_eq!(part2(EXAMPLE.lines().map(|l| l.to_string()), 9), 49);
        assert_eq!(part2(EXAMPLE.lines().map(|l| l.to_string()), 10), 37);
        assert_eq!(part2(EXAMPLE.lines().map(|l| l.to_string()), 100), 2208);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file(), 100);
        println!("{}", res);
        assert_eq!(4036, res);
    }
}
