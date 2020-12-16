use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, ops::RangeInclusive};

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> usize {
    let (rules, _, other_tickets) = parse(input);

    other_tickets
        .iter()
        .flatten()
        .filter(|number| !rules.iter().any(|rule| rule.contains(number)))
        .sum()
}

fn solve_second(input: &str) -> usize {
    let (rules, own_ticket, other_tickets) = parse(input);

    //println!("{:#?}", rules);

    let mut valid_tickets: Vec<Vec<usize>> = other_tickets
        .into_iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|number| rules.iter().any(|rule| rule.contains(number)))
        })
        .collect();
    valid_tickets.push(own_ticket.clone());

    let transposed: Vec<Vec<usize>> = (0..valid_tickets[0].len())
        .map(|index| {
            valid_tickets
                .iter()
                .map(|numbers| numbers[index].clone())
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut matching_rules: Vec<HashSet<String>> = transposed
        .into_iter()
        .map(|numbers| {
            //numbers.sort_unstable();
            //println!("{:?}", numbers);
            let matching: HashSet<String> = rules
                .iter()
                .filter(|rule| numbers.iter().all(|number| rule.contains(number)))
                .map(|rule| rule.name.to_string())
                //.inspect(|rule| println!("{}", rule.name))
                .collect();
            //println!("{:?}", matching.iter().map(|r| r).collect::<Vec<_>>());
            matching
        })
        .collect();

    while !matching_rules.iter().all(|r| r.len() == 1) {
        let solved: HashSet<String> = matching_rules
            .iter()
            .filter(|r| r.len() == 1)
            .map(|r| r.iter().next().unwrap())
            .cloned()
            .collect();
        //println!("{:#?}", solved);

        matching_rules = matching_rules
            .into_iter()
            .map(|r| {
                if r.len() > 1 {
                    r.difference(&solved).cloned().collect()
                } else {
                    r
                }
            })
            .collect();

        //println!("{:#?}", matching_rules);
    }
    let rule_map = matching_rules
        .into_iter()
        .map(|r| r.iter().next().unwrap().clone());

    //println!("{:#?}", matching_rules);

    let departure_values: Vec<usize> = rule_map
        .enumerate()
        .filter(|(_, rule)| rule.starts_with("departure"))
        //.inspect(|(_, rule)| println!("{}", rule.name))
        .map(|(index, _)| own_ticket[index])
        .collect();

    //println!("{:#?}", departure_values);
    assert_eq!(departure_values.len(), 6);
    departure_values.iter().product()
}

lazy_static! {
    static ref BLANK_LINE_REGEX: Regex = Regex::new(r"\n\s*\n").unwrap();
    static ref RULE_REGEX: Regex = Regex::new(
        r"(?P<name>.+): (?P<r1start>\d+)-(?P<r1end>\d+) or (?P<r2start>\d+)-(?P<r2end>\d+)"
    )
    .unwrap();
}

fn parse(input: &str) -> (Vec<Rule<usize>>, Ticket, Vec<Ticket>) {
    let mut parts = BLANK_LINE_REGEX.split(input);
    let rules = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let cap = RULE_REGEX.captures(line).unwrap();

            let r1start = cap["r1start"].parse().unwrap();
            let r1end = cap["r1end"].parse().unwrap();
            let r2start = cap["r2start"].parse().unwrap();
            let r2end = cap["r2end"].parse().unwrap();

            Rule {
                name: cap["name"].to_string(),
                first: r1start..=r1end,
                second: r2start..=r2end,
            }
        })
        .collect();

    let own_ticket = parts
        .next()
        .unwrap()
        .lines()
        .skip(1) // Skip heading.
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let other_tickets = parts
        .next()
        .unwrap()
        .lines()
        .skip(1) // Skip heading.
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, own_ticket, other_tickets)
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule<T: PartialOrd> {
    name: String,
    first: RangeInclusive<T>,
    second: RangeInclusive<T>,
}

impl<T: PartialOrd> Rule<T> {
    fn contains(&self, candidate: &T) -> bool {
        self.first.contains(candidate) || self.second.contains(candidate)
    }
}

type Ticket = Vec<usize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_demo_solution_1() {
        let demo_input = "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50
        
        your ticket:
        7,1,14
        
        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 71);
    }
}
