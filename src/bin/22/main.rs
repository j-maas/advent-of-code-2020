use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    /*let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
    */
}

fn solve_first(input: &str) -> usize {
    let (first_vec, second_vec) = parse(input);
    let mut first = VecDeque::from(first_vec);
    let mut second = VecDeque::from(second_vec);

    while !first.is_empty() && !second.is_empty() {
        let one = first.pop_front().unwrap();
        let two = second.pop_front().unwrap();

        if one > two {
            first.push_back(one);
            first.push_back(two);
        } else {
            second.push_back(two);
            second.push_back(one);
        }
    }

    let winner = if first.is_empty() { second } else { first };

    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(index, number)| (index + 1) * number)
        .sum()
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut parts = BLANK_LINE_REGEX.split(input);

    (
        parse_cards(parts.next().unwrap()),
        parse_cards(parts.next().unwrap()),
    )
}

fn parse_cards(input: &str) -> Vec<usize> {
    let mut lines = input.lines();
    // Discard header.
    lines.next();
    lines
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|number| number.parse().unwrap())
        .collect()
}

lazy_static! {
    static ref BLANK_LINE_REGEX: Regex = Regex::new(r"\n\s*\n").unwrap();
    static ref RULE_ID_REGEX: Regex = Regex::new(r"(?P<id>\d+): (?P<rule>.+)$").unwrap();
    static ref LITERAL_RULE_REGEX: Regex = Regex::new(r#""(?P<literal>.)""#).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_demo_solution_1() {
        let demo_input = "Player 1:
        9
        2
        6
        3
        1
        
        Player 2:
        5
        8
        4
        7
        10";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 306);
    }
}
