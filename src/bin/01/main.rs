use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let first_solution = solve(input, 2);
    println!("The first solution is:\n{}", first_solution);
    let second_solution = solve(input, 3);
    println!("The second solution is:\n{}", second_solution);
}

fn solve(input: &str, size_of_grouping: usize) -> u64 {
    let numbers = parse_numbers(input);
    let summands = find_summands(numbers, size_of_grouping);
    summands.iter().product()
}

fn parse_numbers(input: &str) -> Vec<u64> {
    input.split("\n")
        .map(|entry| entry.trim())
        .filter(|entry| !entry.is_empty())
        .map(|raw| raw.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

fn find_summands(numbers: Vec<u64>, size_of_grouping: usize) -> Vec<u64> {
    let mut combinations = numbers.into_iter().combinations(size_of_grouping);
    combinations.find(|elements| elements.iter().sum::<u64>() == 2020).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO_INPUT: &str =
    "1721
    979
    366
    299
    675
    1456";

    #[test]
    fn first_demo_solution() {
        let solution = solve(DEMO_INPUT, 2);
        assert_eq!(solution, 514579);
    }

    #[test]
    fn second_demo_solution() {
        let solution = solve(DEMO_INPUT, 3);
        assert_eq!(solution, 241861950);
    }
}