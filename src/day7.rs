use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day7.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

fn parse_rules(lines: impl Iterator<Item = String>) -> HashMap<String, Vec<(String, u16)>> {
    HashMap::from_iter(lines.filter(|line| !line.is_empty()).map(|line| {
        if let Some((bag, contents)) = line.split(" bags contain ").collect_tuple() {
            if contents == "no other bags." {
                (bag.to_string(), Vec::new())
            } else {
                let parsed_contents = contents
                    .replace(".", "")
                    .split(", ")
                    .map(|phrase| {
                        let (count_str, inner_bag_phrase) = phrase.split_at(1);
                        let count = u16::from_str(count_str).unwrap();
                        let suffix_len = if count > 1 { 5 } else { 4 };
                        let inner_bag =
                            inner_bag_phrase[1..inner_bag_phrase.len() - suffix_len].to_string();
                        (inner_bag, count)
                    })
                    .collect_vec();
                (bag.to_string(), parsed_contents)
            }
        } else {
            panic!("{}", line);
        }
    }))
}

fn reverse_rules(rules: &HashMap<String, Vec<(String, u16)>>) -> HashMap<&String, Vec<&String>> {
    rules
        .iter()
        .flat_map(|(bag, contents)| {
            contents
                .iter()
                .map(move |(inner_bag, _count)| (inner_bag, bag))
        })
        .fold(HashMap::new(), |mut res, (inner_bag, outer_bag)| {
            if let Some(vec) = res.get_mut(inner_bag) {
                vec.push(outer_bag);
            } else {
                res.insert(inner_bag, vec![outer_bag]);
            }
            res
        })
}

fn collect_outer_bags<'a>(
    reverse_rules: &'a HashMap<&String, Vec<&String>>,
    inner_bag: &String,
    result: &mut HashSet<&'a String>,
) {
    if let Some(outer_bags) = reverse_rules.get(inner_bag) {
        for outer_bag in outer_bags {
            result.insert(*outer_bag);
            collect_outer_bags(reverse_rules, outer_bag, result);
        }
    }
}

fn count_inner_bags(rules: &HashMap<String, Vec<(String, u16)>>, outer_bag: &String) -> u64 {
    match rules.get(outer_bag) {
        None => 0,
        Some(inner_bags) => inner_bags
            .iter()
            .map(|(inner_bag, count)| {
                let inner_bag_count = count_inner_bags(rules, inner_bag);
                (*count as u64) * (inner_bag_count + 1)
            })
            .sum(),
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::{
        collect_outer_bags, count_inner_bags, parse_rules, read_file, reverse_rules,
    };
    use std::collections::HashSet;

    const EXAMPLE1: &str = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

    #[test]
    fn part1_example() {
        let mut outer_bags = HashSet::new();
        let rules = parse_rules(EXAMPLE1[1..].to_string().lines().map(|s| s.to_string()));

        let reverse_rules = reverse_rules(&rules);
        collect_outer_bags(&reverse_rules, &"shiny gold".to_string(), &mut outer_bags);
        let res = outer_bags.len();
        assert_eq!(res, 4);
    }

    #[test]
    fn part1() {
        let mut outer_bags = HashSet::new();
        let rules = parse_rules(read_file());
        let reverse_rules = reverse_rules(&rules);
        collect_outer_bags(&reverse_rules, &"shiny gold".to_string(), &mut outer_bags);
        let res = outer_bags.len();
        println!("{}", res);
        assert_eq!(res, 233);
    }

    #[test]
    fn part2_example1() {
        let rules = parse_rules(EXAMPLE1[1..].to_string().lines().map(|s| s.to_string()));
        println!("{:?}", rules.get(&"faded blue".to_string()));

        let res = count_inner_bags(&rules, &"shiny gold".to_string());
        assert_eq!(res, 32);
    }

    const EXAMPLE2: &str = "
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

    #[test]
    fn part2_example2() {
        let rules = parse_rules(EXAMPLE2[1..].to_string().lines().map(|s| s.to_string()));
        let res = count_inner_bags(&rules, &"shiny gold".to_string());
        assert_eq!(res, 126);
    }

    #[test]
    fn part2() {
        let rules = parse_rules(read_file());
        let res = count_inner_bags(&rules, &"shiny gold".to_string());
        println!("{}", res);
        assert_eq!(res, 421550);
    }
}
