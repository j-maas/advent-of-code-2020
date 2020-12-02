use lazy_static::lazy_static;

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);
    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> usize {
    let entries = parse_input(input);
    let correct_entries = entries.iter().filter(|entry| valid_entry_range(*entry));
    correct_entries.count()
}

fn solve_second(input: &str) -> usize {
    let entries = parse_input(input);
    let correct_entries = entries.iter().filter(|entry| valid_entry_positions(*entry));
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
        r"(?P<first_number>\d+)-(?P<second_number>\d+) (?P<letter>[[:alpha:]]): (?P<password>[[:alpha:]]+)"
    )
    .unwrap();
}

fn parse_line(line: &str) -> Option<Entry> {
    REGEX.captures(line).map(|captures| {
        let first_number = captures
            .name("first_number")
            .and_then(|raw| raw.as_str().parse::<usize>().ok())
            .unwrap();
        let second_number = captures
            .name("second_number")
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
            first_number,
            second_number,
            letter: character,
            password,
        }
    })
}

#[derive(Debug)]
struct Entry {
    first_number: usize,
    second_number: usize,
    letter: char,
    password: String,
}

fn valid_entry_range(entry: &Entry) -> bool {
    let number_of_letters = entry
        .password
        .chars()
        .filter(|c| c == &entry.letter)
        .count();
    let range = entry.first_number..=entry.second_number;

    range.contains(&number_of_letters)
}

fn valid_entry_positions(entry: &Entry) -> bool {
    let first_letter = entry.password.chars().nth(entry.first_number - 1).unwrap(); // 0-indexed
    let second_letter = entry.password.chars().nth(entry.second_number - 1).unwrap(); // 0-indexed

    let mut matches = 0;
    if first_letter == entry.letter {
        matches += 1;
    }
    if second_letter == entry.letter {
        matches += 1;
    }

    matches == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO_INPUT: &str = "1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc";

    #[test]
    fn first_demo_solution() {
        let solution = solve_first(DEMO_INPUT);
        assert_eq!(solution, 2);
    }

    #[test]
    fn second_demo_solution() {
        let solution = solve_second(DEMO_INPUT);
        assert_eq!(solution, 1);
    }
}
