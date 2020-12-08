use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, recognize},
    sequence::tuple,
};
use nom::{combinator::map, IResult};

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> isize {
    let instructions = parse_lines(input);

    let mut acc = 0;
    let mut visited = HashSet::new();
    let mut next_instruction = 0;
    while !visited.contains(&next_instruction) {
        visited.insert(next_instruction);
        let instruction = &instructions[next_instruction];
        use Instruction::*;
        match instruction {
            Nop(_) => next_instruction += 1,
            Acc(increment) => {
                acc += increment;
                next_instruction += 1;
            }
            Jmp(difference) => {
                next_instruction = ((next_instruction as isize) + difference) as usize
            }
        }
    }

    acc
}

fn solve_second(input: &str) -> isize {
    let instructions = parse_lines(input);
    let uncorrupted = uncorrupt(&instructions);

    for ins in uncorrupted {
        let (is_infinite, acc) = infinite_loop(ins);
        if !is_infinite {
            return acc;
        }
    }

    unreachable!();
}

fn infinite_loop(ins: Vec<Instruction>) -> (bool, isize) {
    let mut acc = 0;
    let mut visited = HashSet::new();
    let mut next_instruction = 0;
    while !visited.contains(&next_instruction) && next_instruction < ins.len() {
        visited.insert(next_instruction);
        let instruction = &ins[next_instruction];
        use Instruction::*;
        match instruction {
            Nop(_) => next_instruction += 1,
            Acc(increment) => {
                acc += increment;
                next_instruction += 1;
            }
            Jmp(difference) => {
                next_instruction = ((next_instruction as isize) + difference) as usize
            }
        }
    }

    (visited.contains(&next_instruction), acc)
}

fn parse_lines(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(Instruction::from)
        .collect()
}

#[derive(Debug, Clone)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Instruction {
    fn from(input: &str) -> Instruction {
        let (_, instruction) = alt((nop_parser, acc_parser, jmp_parser))(input).unwrap();
        instruction
    }
}

fn nop_parser(input: &str) -> IResult<&str, Instruction> {
    map(tuple((tag("nop "), number_parser)), |(_, number)| {
        Instruction::Nop(number)
    })(input)
}
fn acc_parser(input: &str) -> IResult<&str, Instruction> {
    map(tuple((tag("acc "), number_parser)), |(_, number)| {
        Instruction::Acc(number)
    })(input)
}
fn jmp_parser(input: &str) -> IResult<&str, Instruction> {
    map(tuple((tag("jmp "), number_parser)), |(_, number)| {
        Instruction::Jmp(number)
    })(input)
}

fn number_parser(input: &str) -> IResult<&str, isize> {
    map_res(
        recognize(tuple((alt((tag("+"), tag("-"))), digit1))),
        str::parse,
    )(input)
}

fn uncorrupt(instructions: &Vec<Instruction>) -> Vec<Vec<Instruction>> {
    instructions
        .iter()
        .enumerate()
        .map(|(index, _)| index)
        .map(|index| {
            let mut cloned: Vec<_> = instructions.iter().cloned().collect();
            use Instruction::*;
            cloned[index] = match cloned[index] {
                Nop(n) => Jmp(n),
                Jmp(n) => Nop(n),
                Acc(n) => Acc(n),
            };
            cloned
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_solution_1() {
        let demo_input = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 5);
    }
    #[test]
    fn demo_solution_2() {
        let demo_input = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 8);
    }
}
