use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryInto;

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(parse_boarding_pass)
        .max()
        .unwrap()
}

fn solve_second(input: &str) -> usize {
    let mut taken_seats = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(parse_boarding_pass)
        .collect::<Vec<_>>();
    taken_seats.sort_unstable();

    for i in 0..(taken_seats.len() - 1) {
        let first = taken_seats[i];
        let next = taken_seats[i + 1];
        if first == next - 2 {
            return next - 1;
        }
    }

    panic!()
}

fn parse_boarding_pass(pass: &str) -> usize {
    let (rows, columns) = parse(pass);
    let row_number = number_from_bits(rows.iter());
    let column_number = number_from_bits(columns.iter());

    row_number * 8 + column_number
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(?P<row>(F|B){7})(?P<column>(L|R){3})").unwrap();
}
fn parse(input: &str) -> ([Bit; 7], [Bit; 3]) {
    REGEX
        .captures(input)
        .map(|cap| (parse_row(&cap["row"]), parse_column(&cap["column"])))
        .unwrap()
}

fn parse_row(input: &str) -> [Bit; 7] {
    input
        .chars()
        .map(|c| search_from_row(&c))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn parse_column(input: &str) -> [Bit; 3] {
    input
        .chars()
        .map(|c| search_from_column(&c))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
#[derive(Debug)]
enum Bit {
    Low,
    High,
}

fn number_from_bit(bit: &Bit) -> usize {
    match bit {
        Bit::Low => 0,
        Bit::High => 1,
    }
}

fn search_from_row(c: &char) -> Bit {
    match c {
        'F' => Bit::Low,
        'B' => Bit::High,
        _ => unreachable!(),
    }
}

fn search_from_column(c: &char) -> Bit {
    match c {
        'L' => Bit::Low,
        'R' => Bit::High,
        _ => unreachable!(),
    }
}

fn number_from_bits<'a>(bits: impl DoubleEndedIterator<Item = &'a Bit>) -> usize {
    bits.into_iter()
        .rfold((0, 0), |(pot, acc), bit| {
            let current = (2usize).pow(pot) * number_from_bit(&bit);
            let new_acc = current + acc;
            (pot + 1, new_acc)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_solution_1() {
        let demo_input = "FBFBBFFRLR";
        let solution = parse_boarding_pass(demo_input);
        assert_eq!(solution, 357);
    }

    #[test]
    fn demo_solution_2() {
        let demo_input = "BFFFBBFRRR";
        let solution = parse_boarding_pass(demo_input);
        assert_eq!(solution, 567);
    }
    #[test]
    fn demo_solution_3() {
        let demo_input = "FFFBBBFRRR";
        let solution = parse_boarding_pass(demo_input);
        assert_eq!(solution, 119);
    }
    #[test]
    fn demo_solution_4() {
        let demo_input = "BBFFBBFRLL";
        let solution = parse_boarding_pass(demo_input);
        assert_eq!(solution, 820);
    }
}
