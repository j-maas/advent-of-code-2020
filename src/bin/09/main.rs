use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input, 25);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input, 25);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str, preamble_size: usize) -> usize {
    let numbers = parse(input);

    for i in preamble_size..numbers.len() {
        let preamble = &numbers[i - preamble_size..i];
        let next = numbers[i];

        if !is_valid(preamble, next) {
            return next;
        }
    }
    unreachable!();
}

fn solve_second(input: &str, preamble_size: usize) -> usize {
    let numbers = parse(input);

    for i in preamble_size..numbers.len() {
        let preamble = &numbers[i - preamble_size..i];
        let next = numbers[i];

        if !is_valid(preamble, next) {
            return find_chain(&numbers[0..i], next);
        }
    }
    unreachable!();
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

fn is_valid(preamble: &[usize], next: usize) -> bool {
    preamble
        .iter()
        .tuple_combinations()
        .any(|(a, b)| a + b == next)
}

fn find_chain(candidates: &[usize], target: usize) -> usize {
    let possible_start = candidates.len() - 1; // We need at least 2 elements.
    for start in 0..possible_start {
        for end in start + 1..candidates.len() {
            let chain = &candidates[start..end];
            let sum: usize = chain.iter().sum();
            if sum == target {
                return chain.iter().min().unwrap() + chain.iter().max().unwrap();
            } else if sum > target {
                break;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_solution_1() {
        let demo_input = "
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        let solution = solve_first(demo_input, 5);
        assert_eq!(solution, 127);
    }
    #[test]
    fn demo_solution_2() {
        let demo_input = "
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";
        let solution = solve_second(demo_input, 5);
        assert_eq!(solution, 62);
    }
}
