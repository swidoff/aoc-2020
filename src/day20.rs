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
    let file = File::open("input/day20.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

#[derive(Clone)]
struct Tile {
    tile_no: u64,
    tile: Vec<Vec<char>>,
}

impl Tile {
    fn dim(&self) -> usize {
        self.tile.len()
    }

    fn empty(&self) -> Tile {
        let mut new_tile = Vec::new();
        for _ in 0..self.dim() {
            let mut row = Vec::with_capacity(self.dim());
            for _ in 0..self.dim() {
                row.push(' ');
            }
            new_tile.push(row)
        }

        Tile {
            tile_no: self.tile_no,
            tile: new_tile,
        }
    }

    fn rotate(&self) -> Tile {
        let mut new_tile = self.empty();

        for r in 0..self.dim() {
            for c in 0..self.dim() {
                new_tile.tile[c][self.dim() - r - 1] = self.tile[r][c];
            }
        }
        new_tile
    }

    fn flip(&self) -> Tile {
        let mut new_tile = self.empty();
        for r in 0..self.dim() {
            for c in 0..self.dim() {
                new_tile.tile[self.dim() - r - 1][c] = self.tile[r][c];
            }
        }
        new_tile
    }
}

fn parse_tiles(iter: impl Iterator<Item = String>) -> Vec<Tile> {
    let mut tile = Some(Vec::new());
    let mut tile_no = 0;
    let mut res = Vec::new();

    for line in iter {
        if line.is_empty() {
            if let Some(v) = tile.replace(Vec::new()) {
                res.push(Tile { tile_no, tile: v });
            }
        } else if line.starts_with("Tile") {
            tile_no = u64::from_str(
                line.strip_prefix("Tile ")
                    .unwrap()
                    .strip_suffix(":")
                    .unwrap(),
            )
            .unwrap();
        } else if let Some(v) = tile.as_mut() {
            v.push(line.chars().collect_vec());
        }
    }

    if let Some(v) = tile.replace(Vec::new()) {
        res.push(Tile { tile_no, tile: v });
    }
    res
}

fn border_coords(side: usize, dim: usize) -> Vec<(usize, usize)> {
    match side {
        1 => (0..dim).map(|c| (0, c)).collect_vec(),
        2 => (0..dim).map(|r| (r, dim - 1)).collect_vec(),
        3 => (0..dim).map(|c| (dim - 1, c)).collect_vec(),
        _ => (0..dim).map(|r| (r, 0)).collect_vec(),
    }
}

fn find_matches(tile1: &Tile, tile2: &Tile) -> Vec<usize> {
    let mut res = Vec::new();
    let dim = tile1.tile.len();
    for side1 in 1..5 {
        for side2 in 1..5 {
            for reverse in [false, true].iter() {
                let coords1 = border_coords(side1, dim);
                let mut coords2 = border_coords(side2, dim);
                if *reverse {
                    coords2.reverse();
                }

                let matches = coords1
                    .iter()
                    .zip(coords2.iter())
                    .all(|((r1, c1), (r2, c2))| tile1.tile[*r1][*c1] == tile2.tile[*r2][*c2]);
                if matches {
                    res.push(side1);
                }
            }
        }
    }
    res
}

fn find_upper_left_corner(tiles: &Vec<Tile>) -> &Tile {
    tiles
        .iter()
        .find_map(|tile| {
            let mut res = Vec::new();

            for other_tile in tiles.iter() {
                if other_tile.tile_no != tile.tile_no {
                    let mut matches = find_matches(tile, other_tile);
                    res.append(&mut matches);
                }
            }

            if res.len() == 2 && res.contains(&2) && res.contains(&3) {
                Some(tile)
            } else {
                None
            }
        })
        .unwrap()
}

fn match_side(tile1: &Tile, tile2: &Tile, side1: usize, side2: usize) -> Option<Tile> {
    let coords1 = border_coords(side1, tile1.dim());
    let coords2 = border_coords(side2, tile2.dim());
    let mut tile2 = tile2.clone();

    for flip in 0..2 {
        if flip == 1 {
            tile2 = tile2.flip();
        }

        for _ in 0..4 {
            let matches = coords1
                .iter()
                .zip(coords2.iter())
                .all(|((r1, c1), (r2, c2))| tile1.tile[*r1][*c1] == tile2.tile[*r2][*c2]);

            if matches {
                return Some(tile2);
            }

            tile2 = tile2.rotate();
        }
    }
    None
}

fn arrange_tiles(tiles: &Vec<Tile>, dim: usize) -> Vec<Vec<Tile>> {
    let mut res: Vec<Vec<Tile>> = Vec::with_capacity(dim);
    let mut remaining_tiles: HashMap<u64, &Tile> =
        HashMap::from_iter(tiles.iter().map(|tile| (tile.tile_no, tile)));

    let upper_left = find_upper_left_corner(&tiles);
    remaining_tiles.remove(&upper_left.tile_no);

    for r in 0..dim {
        let mut row = Vec::with_capacity(dim);
        if r == 0 {
            row.push(upper_left.clone())
        } else {
            let tile = remaining_tiles
                .values()
                .find_map(|tile| match_side(&res[r - 1][0], *tile, 3, 1))
                .unwrap();
            remaining_tiles.remove(&tile.tile_no);
            row.push(tile);
        }

        for c in 1..dim {
            let tile = remaining_tiles
                .values()
                .find_map(|tile| match_side(&row[c - 1], *tile, 2, 4))
                .unwrap();
            remaining_tiles.remove(&tile.tile_no);
            row.push(tile);
        }

        res.push(row)
    }

    res
}

fn merge_tiles(tiles: &Vec<Vec<Tile>>) -> Tile {
    let inner_dim = tiles[0][0].dim() - 2;
    let dim = (inner_dim) * tiles.len();
    let mut big_tile = Vec::with_capacity(dim);
    for _ in 0..dim {
        let mut row = Vec::with_capacity(dim);
        for _ in 0..dim {
            row.push(' ');
        }
        big_tile.push(row);
    }

    for (big_r, row) in tiles.iter().enumerate() {
        for (big_c, little_tile) in row.iter().enumerate() {
            for little_r in 1..little_tile.dim() - 1 {
                for little_c in 1..little_tile.dim() - 1 {
                    let ch = little_tile.tile[little_r][little_c];
                    big_tile[big_r * inner_dim + little_r - 1][big_c * inner_dim + little_c - 1] =
                        ch;
                }
            }
        }
    }

    Tile {
        tile_no: 0,
        tile: big_tile,
    }
}

fn find_sea_monsters(tile: &Tile) -> usize {
    let sea_monster = "
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
";

    let mask = sea_monster[1..]
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut sea_monsters = 0;
    let mut image = tile.clone();

    for flip in 0..2 {
        if flip == 1 {
            image = image.flip();
        }

        for _ in 0..4 {
            for r in 0..image.tile.len() - mask.len() {
                for c in 0..image.tile.len() - mask[0].len() {
                    if mask_match(&image.tile, r, c, &mask) {
                        sea_monsters += 1;
                    }
                }
            }

            image = image.rotate();
        }
    }

    count_hashes(&tile.tile) - sea_monsters * count_hashes(&mask)
}

fn count_hashes(tile: &Vec<Vec<char>>) -> usize {
    tile.iter()
        .flat_map(|v| v.iter())
        .filter(|c| **c == '#')
        .count()
}

fn mask_match(tile: &Vec<Vec<char>>, r: usize, c: usize, mask: &Vec<Vec<char>>) -> bool {
    for mask_r in 0..mask.len() {
        for mask_c in 0..mask[0].len() {
            let mask_char = mask[mask_r][mask_c];
            let tile_char = tile[r + mask_r][c + mask_c];
            if mask_char == '#' && tile_char != '#' {
                return false;
            }
        }
    }
    true
}

mod tests {
    use crate::day20::{
        arrange_tiles, count_all_matches, find_sea_monsters, find_upper_left_corner, merge_tiles,
        parse_tiles, read_file,
    };
    use itertools::Itertools;

    const EXAMPLE: &str = "
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
";

    #[test]
    fn test_part1_example() {
        let tiles = parse_tiles(EXAMPLE[1..].lines().map(|line| line.to_string()));
        let res = arrange_tiles(&tiles, 3);
        let res = res[0][0].tile_no * res[0][2].tile_no * res[2][0].tile_no * res[2][2].tile_no;
        assert_eq!(20899048083289, res);
    }

    #[test]
    fn test_part1() {
        let tiles = parse_tiles(read_file());
        let res = arrange_tiles(&tiles, 12);
        let res = res[0][0].tile_no * res[0][11].tile_no * res[11][0].tile_no * res[11][11].tile_no;
        assert_eq!(7901522557967, res);
    }

    #[test]
    fn test_part2_example() {
        let tiles = parse_tiles(EXAMPLE[1..].lines().map(|line| line.to_string()));
        let res = arrange_tiles(&tiles, 3);
        let res = merge_tiles(&res);
        let res = find_sea_monsters(&res);
        assert_eq!(273, res);
    }

    #[test]
    fn test_part2() {
        let tiles = parse_tiles(read_file());
        let res = arrange_tiles(&tiles, 12);
        let res = merge_tiles(&res);
        let res = find_sea_monsters(&res);
        println!("{}", res);
        // assert_eq!(273, res);
    }
}
