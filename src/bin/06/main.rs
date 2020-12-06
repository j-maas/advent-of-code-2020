use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> usize {
    let groups = parse_first(input);
    groups.iter().map(|group| group.len()).sum()
}

lazy_static! {
    static ref BLANK_LINE_REGEX: Regex = Regex::new(r"\n\s*\n").unwrap();
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"[\s\n]*").unwrap();
}

fn parse_first(input: &str) -> Vec<HashSet<char>> {
    BLANK_LINE_REGEX
        .split(input)
        .map(|line| WHITESPACE_REGEX.replace_all(line, ""))
        .filter(|line| !line.is_empty())
        .map(|group| group.chars().collect::<HashSet<_>>())
        .collect()
}

fn solve_second(input: &str) -> usize {
    let groups = parse_second(input);
    groups.iter().map(|group| group.len()).sum()
}

fn parse_second(input: &str) -> Vec<HashSet<char>> {
    BLANK_LINE_REGEX
        .split(input)
        .map(|group_lines| {
            let mut group = group_lines
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .map(|line| line.chars().collect::<HashSet<_>>());
            let first = group.next().unwrap();
            group.fold(first, |acc, next| {
                acc.intersection(&next).cloned().collect()
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_solution_1() {
        let demo_input = "abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 11);
    }
}
