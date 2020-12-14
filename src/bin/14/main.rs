use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> u64 {
    let instructions = parse(input);
    let mut instructions_iter = instructions.iter();

    let mut mask = if let Instruction::Mask(mask) = instructions_iter.next().unwrap() {
        mask
    } else {
        unreachable!()
    };
    let mut memory = HashMap::new();

    for i in instructions_iter {
        match i {
            Instruction::Mask(new_mask) => mask = new_mask,
            Instruction::Mem(address, value) => {
                let masked_value = mask.apply(*value);
                memory.insert(address, masked_value);
            }
        }
    }

    memory.values().sum()
}

fn solve_second(input: &str) -> u64 {
    let instructions = parse2(input);
    let mut instructions_iter = instructions.iter();

    let mut mask = if let Instruction2::Mask(mask) = instructions_iter.next().unwrap() {
        mask
    } else {
        unreachable!()
    };
    let mut memory = HashMap::new();

    for i in instructions_iter {
        match i {
            Instruction2::Mask(new_mask) => mask = new_mask,
            Instruction2::Mem(address, value) => {
                let addresses = mask.apply(*address);
                for a in addresses {
                    memory.insert(a, *value);
                }
            }
        }
    }

    memory.values().sum()
}
lazy_static! {
    static ref MASK_REGEX: Regex = Regex::new(r"mask = (?P<mask>[X01]{36})").unwrap();
    static ref MEM_REGEX: Regex = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            if let Some(cap) = MASK_REGEX.captures(line) {
                return Instruction::Mask(Mask::from_str(&cap["mask"]).unwrap());
            }

            if let Some(cap) = MEM_REGEX.captures(line) {
                return Instruction::Mem(
                    cap["address"].parse().unwrap(),
                    cap["value"].parse().unwrap(),
                );
            }

            unreachable!();
        })
        .collect()
}

fn parse2(input: &str) -> Vec<Instruction2> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            if let Some(cap) = MASK_REGEX.captures(line) {
                return Instruction2::Mask(Mask2::from_str(&cap["mask"]).unwrap());
            }

            if let Some(cap) = MEM_REGEX.captures(line) {
                return Instruction2::Mem(
                    cap["address"].parse().unwrap(),
                    cap["value"].parse().unwrap(),
                );
            }

            unreachable!();
        })
        .collect()
}
#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    Mem(u64, u64),
}

#[derive(Debug)]
enum Instruction2 {
    Mask(Mask2),
    Mem(u64, u64),
}

#[derive(Debug)]
struct Mask {
    ones: u64,
    zeroes: u64,
}

impl FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .rev()
            .map(|c| {
                if c == 'X' {
                    return Ok(None);
                }

                let digit = c.to_digit(10);

                digit
                    .map(|digit| if digit > 1 { None } else { Some(digit) })
                    .ok_or(())
            })
            // We will invert the zeroes in the next step.
            .try_fold((0, 0, 0), |(ones, zeroes, pow), next| match next {
                Ok(Some(1)) => Ok((ones | (1 << pow), zeroes, pow + 1)),
                Ok(Some(0)) => Ok((ones, zeroes | (1 << pow), pow + 1)),
                Ok(None) => Ok((ones, zeroes, pow + 1)),
                _ => Err(()),
            })
            .map(|(ones, zeroes, _)| Mask {
                ones,
                zeroes: u64::MAX - zeroes,
            })
    }
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        (value | self.ones) & self.zeroes
    }

    fn new() -> Self {
        Self {
            ones: 0,
            zeroes: u64::MAX,
        }
    }

    fn set_one_at(&mut self, pow: u64) {
        self.ones |= 1 << pow;
        self.zeroes |= 1 << pow;
    }

    fn set_zero_at(&mut self, pow: u64) {
        self.zeroes &= u64::MAX - 1 << pow;
        self.ones &= u64::MAX - 1 << pow;
    }
}
#[derive(Debug)]
struct Mask2 {
    ones: u64,
    floating: HashSet<u64>,
}

impl FromStr for Mask2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .rev()
            .map(|c| {
                if c == 'X' {
                    return Ok(None);
                }

                let digit = c.to_digit(10);

                digit
                    .map(|digit| if digit > 1 { None } else { Some(digit) })
                    .ok_or(())
            })
            // We will invert the zeroes in the next step.
            .try_fold(
                (0, HashSet::new(), 0),
                |(ones, mut floating, pow), next| match next {
                    Ok(Some(1)) => Ok((ones | (1 << pow), floating, pow + 1)),
                    Ok(None) => {
                        floating.insert(pow);
                        Ok((ones, floating, pow + 1))
                    }
                    Ok(Some(0)) => Ok((ones, floating, pow + 1)),
                    _ => Err(()),
                },
            )
            .map(|(ones, floating, _)| Mask2 { ones, floating })
    }
}
impl Mask2 {
    fn apply(&self, value: u64) -> Vec<u64> {
        let masked = value | self.ones;

        let mut results = vec![masked];

        for f in &self.floating {
            results = results
                .iter()
                .flat_map(|m| vec![m | (1 << f), m & (u64::MAX - (1 << f))])
                .collect();
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_demo_solution_1() {
        let demo_input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 101 + 64);
    }

    #[test]
    fn first_demo_solution_2() {
        let demo_input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 2";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 101 + 64);
    }
    #[test]
    fn second_demo_solution_1() {
        let demo_input = "mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 208);
    }
}
