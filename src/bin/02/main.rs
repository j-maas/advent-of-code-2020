use lazy_static::lazy_static;

use regex::Regex;
use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("./input.txt");
    let solution = solve(input);
    println!("The first solution is:\n{}", solution);
}

fn solve(input: &str) -> usize {
    let entries = parse_input(input);
    let correct_entries = entries.into_iter().filter(valid_entry);
    correct_entries.count()
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line).unwrap())
        .collect()
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(
        r"(?P<lower_bound>\d+)-(?P<upper_bound>\d+) (?P<letter>[[:alpha:]]): (?P<password>[[:alpha:]]+)"
    )
    .unwrap();
}

fn parse_line(line: &str) -> Option<Entry> {
    REGEX.captures(line).map(|captures| {
        let lower_bound = captures
            .name("lower_bound")
            .and_then(|raw| raw.as_str().parse::<usize>().ok())
            .unwrap();
        let upper_bound = captures
            .name("upper_bound")
            .and_then(|raw| raw.as_str().parse::<usize>().ok())
            .unwrap();
        let character = captures
            .name("letter")
            .and_then(|m| m.as_str().chars().next())
            .unwrap();
        let password = captures
            .name("password")
            .map(|m| m.as_str().to_string())
            .unwrap();

        Entry {
            range: lower_bound..=upper_bound,
            letter: character,
            password,
        }
    })
}

#[derive(Debug)]
struct Entry {
    range: RangeInclusive<usize>,
    letter: char,
    password: String,
}

fn valid_entry(entry: &Entry) -> bool {
    let number_of_letters = entry
        .password
        .chars()
        .filter(|c| c == &entry.letter)
        .count();

    entry.range.contains(&number_of_letters)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO_INPUT: &str = "1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc";

    #[test]
    fn first_demo_solution() {
        let solution = solve(DEMO_INPUT);
        assert_eq!(solution, 2);
    }
}
