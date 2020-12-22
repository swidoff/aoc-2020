use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day19.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

enum Rule {
    Terminal(char),
    SubRule(Vec<Vec<u64>>),
}

fn parse_input(
    iter: impl Iterator<Item = String>,
    part2: bool,
) -> (HashMap<u64, Rule>, Vec<Vec<char>>) {
    let mut rules = HashMap::new();
    let mut messages = Vec::new();
    let mut parse_rules = true;
    for line in iter {
        if line.is_empty() {
            parse_rules = false
        } else if parse_rules {
            let (index_str, rule_str) = line.split(": ").collect_tuple().unwrap();
            let index = u64::from_str(index_str).unwrap();
            let rule = if rule_str.starts_with("\"") {
                Rule::Terminal(rule_str.chars().nth(1).unwrap())
            } else {
                Rule::SubRule(
                    rule_str
                        .split(" | ")
                        .map(|r| {
                            r.split(" ")
                                .map(|n| u64::from_str(n).unwrap())
                                .collect_vec()
                        })
                        .collect_vec(),
                )
            };
            rules.insert(index, rule);
        } else {
            messages.push(line.chars().collect_vec());
        }
    }

    // Blow out rule 0 to 0: 42+ 42*n 13*n to remove loops.
    // Rules 8 and 11 are now unnecessary.
    if part2 {
        let reps = 5;
        let mut sub_rules = Vec::new();
        for i in 1..reps {
            for j in (i + 1)..(i + 1 + reps) {
                let mut v: Vec<u64> = Vec::new();
                for _ in 0..j {
                    v.push(42);
                }
                for _ in 0..i {
                    v.push(31);
                }
                sub_rules.push(v);
            }
        }

        sub_rules.reverse();
        rules.insert(0, Rule::SubRule(sub_rules));
    }

    (rules, messages)
}

fn is_valid(msg: &Vec<char>, rules: &HashMap<u64, Rule>) -> bool {
    let res = match is_valid_for_rule(&msg, &rules, 0, 0) {
        Some(i) if i == msg.len() => true,
        _ => false,
    };
    // println!("{:?}, {}", msg, res);
    res
}

fn is_valid_for_rule(
    msg: &Vec<char>,
    rules: &HashMap<u64, Rule>,
    rule_index: u64,
    char_index: usize,
) -> Option<usize> {
    // println!("{} {}", rule_index, char_index);
    let res = match rules.get(&rule_index).unwrap() {
        Rule::Terminal(c) => match msg.get(char_index) {
            Some(m) if m == c => Some(char_index + 1),
            _ => None,
        },
        Rule::SubRule(sub_rules) => sub_rules.iter().find_map(|sub_rule| {
            let mut new_char_index = Some(char_index);
            for sub_rule_index in sub_rule {
                new_char_index = match new_char_index {
                    Some(i) => is_valid_for_rule(msg, rules, *sub_rule_index, i),
                    None => break,
                }
            }
            new_char_index
        }),
    };
    res
}

#[cfg(test)]
mod tests {
    use crate::day19::{is_valid, parse_input, read_file};

    #[test]
    fn test_part1_example() {
        let example = "
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
";
        let (rules, messages) =
            parse_input(example[1..].lines().map(|line| line.to_string()), false);
        let res = messages.iter().filter(|msg| is_valid(*msg, &rules)).count();
        assert_eq!(2, res);
    }

    #[test]
    fn test_part1() {
        let (rules, messages) = parse_input(read_file(), false);
        let res = messages.iter().filter(|msg| is_valid(*msg, &rules)).count();
        println!("{}", res);
        assert_eq!(173, res);
    }

    const EXAMPLE2: &str = "
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
";

    #[test]
    fn test_part2_example1() {
        let (rules, messages) =
            parse_input(EXAMPLE2[1..].lines().map(|line| line.to_string()), false);
        let res = messages.iter().filter(|msg| is_valid(*msg, &rules)).count();
        assert_eq!(3, res);
    }

    #[test]
    fn test_part2_example2() {
        let (rules, messages) =
            parse_input(EXAMPLE2[1..].lines().map(|line| line.to_string()), true);
        assert_eq!(true, is_valid(&messages[2], &rules));

        let res = messages.iter().filter(|msg| is_valid(*msg, &rules)).count();
        assert_eq!(12, res);
    }

    #[test]
    fn test_part2() {
        let (rules, messages) = parse_input(read_file(), true);
        let res = messages.iter().filter(|msg| is_valid(*msg, &rules)).count();
        println!("{}", res);
        assert_eq!(367, res);
    }
}
