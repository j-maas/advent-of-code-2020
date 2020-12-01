use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let solution = solve(input);
    println!("The solution is:\n{}", solution);
}

fn solve(input: &str) -> u64 {
    let numbers = parse_numbers(input);
    let (first, second) = find_summands(numbers);
    first * second
}

fn parse_numbers(input: &str) -> Vec<u64> {
    input.split("\n")
        .map(|entry| entry.trim())
        .filter(|entry| !entry.is_empty())
        .map(|raw| raw.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

fn find_summands(numbers: Vec<u64>) -> (u64, u64) {
    let mut combinations = numbers.into_iter().tuple_combinations();
    combinations.find(|(first, second)| first + second == 2020).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_input() {
        let demo_input = "1721
        979
        366
        299
        675
        1456";

        let solution = solve(demo_input);
        assert_eq!(solution, 514579);
    }
}