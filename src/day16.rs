use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> String {
    let file = File::open("input/day16.txt").unwrap();
    let mut res = String::new();
    BufReader::new(file).read_to_string(&mut res).unwrap();
    res
}

struct Notes {
    fields: Vec<(String, u64, u64, u64, u64)>,
    my_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

impl Notes {
    fn is_valid_for_any_field(&self, num: u64) -> bool {
        self.fields.iter().any(|(_, min1, max1, min2, max2)| {
            (num >= *min1 && num <= *max1) || (num >= *min2 && num <= *max2)
        })
    }
}

fn parse_file(file: &String) -> Notes {
    let (part1, part2, part3) = file.split("\n\n").collect_tuple().unwrap();

    let fields = part1
        .lines()
        .map(|line| {
            let (field, ranges) = line.split(": ").collect_tuple().unwrap();
            let (range1, range2) = ranges.split(" or ").collect_tuple().unwrap();
            let (min1, max1) = range1
                .split("-")
                .map(|num| u64::from_str(num).unwrap())
                .collect_tuple()
                .unwrap();
            let (min2, max2) = range2
                .split("-")
                .map(|num| u64::from_str(num).unwrap())
                .collect_tuple()
                .unwrap();
            (field.to_string(), min1, max1, min2, max2)
        })
        .collect_vec();

    let my_ticket = part2
        .lines()
        .skip(1)
        .flat_map(|line| line.split(","))
        .map(|num| u64::from_str(num).unwrap())
        .collect_vec();

    let nearby_tickets = part3
        .lines()
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|num| u64::from_str(num).unwrap())
                .collect_vec()
        })
        .collect_vec();

    Notes {
        fields,
        my_ticket,
        nearby_tickets,
    }
}

// Part 1
fn ticket_scanning_error_rate(notes: &Notes) -> u64 {
    notes
        .nearby_tickets
        .iter()
        .flat_map(|ticket| ticket.iter())
        .filter(|value| !notes.is_valid_for_any_field(**value))
        .sum()
}

// Part 2
fn assign_fields(notes: &Notes) -> Vec<&String> {
    let nearby_valid = notes
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|value| notes.is_valid_for_any_field(*value))
        })
        .collect_vec();

    // For each ticket index, find candidate fields. Multiple fields can be candidates.
    let mut candidates: Vec<HashSet<&String>> = (0..notes.fields.len())
        .map(|field_index| {
            // Find field names for which all tickets are valid for the values at the field index.
            let field_names =
                notes
                    .fields
                    .iter()
                    .filter_map(|(field_name, min1, max1, min2, max2)| {
                        let matches = nearby_valid.iter().all(|ticket| {
                            let value = ticket[field_index];
                            (value >= *min1 && value <= *max1) || (value >= *min2 && value <= *max2)
                        });

                        if matches {
                            Some(field_name)
                        } else {
                            None
                        }
                    });

            HashSet::from_iter(field_names)
        })
        .collect_vec();

    // Remove unique field assignments from the other sets.
    // Continue until all sets are unique.
    loop {
        let singletons = candidates
            .iter()
            .filter_map(|fields| {
                if fields.len() == 1 {
                    Some(*fields.iter().next().unwrap())
                } else {
                    None
                }
            })
            .collect_vec();

        if singletons.len() == candidates.len() {
            break;
        }

        for field in singletons {
            for v in candidates.iter_mut() {
                if v.len() > 1 {
                    v.remove(field);
                }
            }
        }
    }

    candidates
        .iter()
        .map(|v| *v.iter().next().unwrap())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::day16::{assign_fields, parse_file, read_file, ticket_scanning_error_rate};
    use itertools::Itertools;

    #[test]
    fn test_part1_example() {
        let example = "
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
        let notes = parse_file(&example[1..].to_string());
        let res = ticket_scanning_error_rate(&notes);
        assert_eq!(res, 71);
    }

    #[test]
    fn test_part1() {
        let file = read_file();
        let notes = parse_file(&file);
        let res = ticket_scanning_error_rate(&notes);
        println!("{}", res);
    }

    #[test]
    fn test_part2_example() {
        let example = "
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";
        let notes = parse_file(&example[1..].to_string());
        let res = assign_fields(&notes);
        println!("{:?}", res);
        assert_eq!(res, vec!["row", "class", "seat"]);
    }

    #[test]
    fn test_part2() {
        let file = read_file();
        let notes = parse_file(&file);
        let field_order = assign_fields(&notes);

        let res: u64 = field_order
            .iter()
            .enumerate()
            .filter_map(|(i, field)| {
                if field.starts_with("departure") {
                    Some(notes.my_ticket[i])
                } else {
                    None
                }
            })
            .product();

        println!("{}", res);
    }
}
