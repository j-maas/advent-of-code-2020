extern crate pest;
#[macro_use]
extern crate pest_derive;

use lazy_static::lazy_static;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

#[derive(Parser)]
#[grammar = "bin/18/grammar.pest"]
struct MyParser;

lazy_static! {
    static ref PREC_CLIMBER_SAME: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(multiply, Left),
        ])
    };
    static ref PREC_CLIMBER_ORDERED: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(multiply, Left),
            Operator::new(add, Left),
        ])
    };
}

fn solve_first(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let pairs: Pairs<Rule> = MyParser::parse(Rule::calculation, line).unwrap();
            eval(pairs, &PREC_CLIMBER_SAME)
        })
        .sum()
}

fn eval(expression: Pairs<Rule>, climber: &PrecClimber<Rule>) -> isize {
    climber.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<isize>().unwrap(),
            Rule::expr => eval(pair.into_inner(), &climber),
            _ => unreachable!(),
        },
        |lhs: isize, op: Pair<Rule>, rhs: isize| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::multiply => lhs * rhs,
            _ => unreachable!(),
        },
    )
}

fn solve_second(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let pairs: Pairs<Rule> = MyParser::parse(Rule::calculation, line).unwrap();
            eval(pairs, &PREC_CLIMBER_ORDERED)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_demo_solution_1() {
        let demo_input = "2 * 3 + (4 * 5)";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 26);
    }

    #[test]
    fn first_demo_solution_2() {
        let demo_input = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 437);
    }

    #[test]
    fn first_demo_solution_3() {
        let demo_input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 12240);
    }

    #[test]
    fn first_demo_solution_4() {
        let demo_input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 13632);
    }
}
