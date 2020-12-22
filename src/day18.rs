use itertools::Itertools;
use std::borrow::Borrow;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day18.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap().to_string())
}

#[derive(Copy, Clone)]
enum Operator {
    Plus,
    Mult,
}

enum Token {
    Op(Operator),
    Number(u64),
    LeftParen,
    RightParen,
}

fn tokenize(line: &str) -> Vec<Token> {
    line.chars()
        .filter(|c| !c.is_whitespace())
        .map(|s| match s {
            '+' => Token::Op(Operator::Plus),
            '*' => Token::Op(Operator::Mult),
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            x => Token::Number(x.to_digit(10).unwrap() as u64),
        })
        .collect_vec()
}

enum Node {
    Number(u64),
    Expr(Box<Node>, Operator, Box<Node>),
}

fn expression_part1(tokens: &mut VecDeque<Token>) -> Box<Node> {
    let mut res = factor_part1(tokens);
    while let Some(Token::Op(op)) = tokens.front() {
        let op = op.clone();
        tokens.pop_front();
        res = Box::new(Node::Expr(res, op, factor_part1(tokens)));
    }
    res
}

fn expression_part2(tokens: &mut VecDeque<Token>) -> Box<Node> {
    let mut res = term(tokens);
    while let Some(Token::Op(Operator::Mult)) = tokens.front() {
        tokens.pop_front();
        res = Box::new(Node::Expr(res, Operator::Mult, term(tokens)));
    }
    res
}

fn term(tokens: &mut VecDeque<Token>) -> Box<Node> {
    let mut res = factor_part2(tokens);
    while let Some(Token::Op(Operator::Plus)) = tokens.front() {
        tokens.pop_front();
        res = Box::new(Node::Expr(res, Operator::Plus, factor_part2(tokens)));
    }
    res
}

fn factor_part1(tokens: &mut VecDeque<Token>) -> Box<Node> {
    match tokens.pop_front() {
        Some(Token::Number(n)) => Box::new(Node::Number(n)),
        Some(Token::LeftParen) => {
            let res = expression_part1(tokens);
            tokens.pop_front();
            res
        }
        _ => panic!("Bad expression"),
    }
}

fn factor_part2(tokens: &mut VecDeque<Token>) -> Box<Node> {
    match tokens.pop_front() {
        Some(Token::Number(n)) => Box::new(Node::Number(n)),
        Some(Token::LeftParen) => {
            let res = expression_part2(tokens);
            tokens.pop_front();
            res
        }
        _ => panic!("Bad expression"),
    }
}

fn evaluate_node(node: &Box<Node>) -> u64 {
    match node.borrow() {
        Node::Number(n) => *n,
        Node::Expr(lhs, Operator::Plus, rhs) => evaluate_node(lhs) + evaluate_node(rhs),
        Node::Expr(lhs, Operator::Mult, rhs) => evaluate_node(lhs) * evaluate_node(rhs),
    }
}

fn evaluate_part1(tokens: Vec<Token>) -> u64 {
    let mut inputs = VecDeque::from(tokens);
    let node = expression_part1(&mut inputs);
    evaluate_node(&node)
}

fn evaluate_part2(tokens: Vec<Token>) -> u64 {
    let mut inputs = VecDeque::from(tokens);
    let node = expression_part2(&mut inputs);
    evaluate_node(&node)
}

#[cfg(test)]
mod tests {
    use crate::day18::{evaluate_part1, evaluate_part2, read_file, tokenize};

    #[test]
    fn test_part1_example() {
        assert_eq!(71, evaluate_part1(tokenize("1 + 2 * 3 + 4 * 5 + 6")));
        assert_eq!(51, evaluate_part1(tokenize("1 + (2 * 3) + (4 * (5 + 6))")));
        assert_eq!(26, evaluate_part1(tokenize("2 * 3 + (4 * 5)")));
        assert_eq!(437, evaluate_part1(tokenize("5 + (8 * 3 + 9 + 3 * 4 * 3)")));
        assert_eq!(
            12240,
            evaluate_part1(tokenize("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"))
        );
        assert_eq!(
            13632,
            evaluate_part1(tokenize("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"))
        );
    }

    #[test]
    fn test_part1() {
        let res: u64 = read_file()
            .map(|line| tokenize(line.as_str()))
            .map(|inputs| evaluate_part1(inputs))
            .sum();
        println!("{}", res);
        assert_eq!(202553439706, res);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(231, evaluate_part2(tokenize("1 + 2 * 3 + 4 * 5 + 6")));
        assert_eq!(51, evaluate_part2(tokenize("1 + (2 * 3) + (4 * (5 + 6))")));
        assert_eq!(46, evaluate_part2(tokenize("2 * 3 + (4 * 5)")));
        assert_eq!(
            1445,
            evaluate_part2(tokenize("5 + (8 * 3 + 9 + 3 * 4 * 3)"))
        );
        assert_eq!(
            669060,
            evaluate_part2(tokenize("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"))
        );
        assert_eq!(
            23340,
            evaluate_part2(tokenize("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"))
        );
    }

    #[test]
    fn test_part2() {
        let res: u64 = read_file()
            .map(|line| tokenize(line.as_str()))
            .map(|inputs| evaluate_part2(inputs))
            .sum();
        println!("{}", res);
        assert_eq!(88534268715686, res);
    }
}
