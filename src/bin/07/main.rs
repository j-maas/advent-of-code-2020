use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input, &Bag::from("shiny", "gold"));
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input, &Bag::from("shiny", "gold"));
    println!("The second solution is:\n{}", second_solution);
}
fn solve_first(input: &str, target: &Bag) -> usize {
    let bag_dependencies = parse_dependencies(input);

    let mut inverse_deps = HashMap::new();
    bag_dependencies.iter().for_each(|(bag, deps)| {
        deps.iter().for_each(|(_, dep)| {
            inverse_deps
                .entry(dep)
                .or_insert(HashSet::new())
                .insert(bag);
        });
    });

    let mut result = HashSet::new();
    let mut to_visit = inverse_deps.get(target).unwrap().clone();

    while !to_visit.is_empty() {
        result = result.union(&to_visit).cloned().collect();

        let mut to_visit_next = HashSet::new();
        to_visit.clone().iter().for_each(|node| {
            if let Some(deps) = inverse_deps.get(node) {
                to_visit_next = to_visit_next.union(deps).cloned().collect();
            }
        });
        to_visit = to_visit_next;
    }

    result.len()
}

fn solve_second(input: &str, target: &Bag) -> usize {
    let bag_dependencies = parse_dependencies(input);

    bag_dependencies
        .get(target)
        .unwrap()
        .iter()
        .map(|(count, dep)| count + (count * count_deps(dep, &bag_dependencies)))
        .sum()
}

fn count_deps(target: &Bag, deps: &HashMap<Bag, Vec<(usize, Bag)>>) -> usize {
    deps.get(target)
        .map(|d| {
            d.iter()
                .map(|(count, dep)| count + (count * count_deps(dep, deps)))
                .sum()
        })
        .unwrap_or(1)
}

fn parse_dependencies(input: &str) -> HashMap<Bag, Vec<(usize, Bag)>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect()
}

lazy_static! {
    static ref FIRST_BAG_REGEX: Regex =
        Regex::new(r"(?P<adj>[a-z]+) (?P<color>[a-z]+) bags").unwrap();
    static ref DEP_BAG_REGEX: Regex =
        Regex::new(r"(?P<count>\d+) (?P<adj>[a-z]+) (?P<color>[a-z]+) bag").unwrap();
}

fn parse_line(input: &str) -> (Bag, Vec<(usize, Bag)>) {
    let first_bag_cap = FIRST_BAG_REGEX.captures(input).unwrap();
    let first_bag = Bag::from(&first_bag_cap["adj"], &first_bag_cap["color"]);

    let other_bags = DEP_BAG_REGEX
        .captures_iter(input)
        .map(|cap| {
            (
                cap["count"].parse::<usize>().unwrap(),
                Bag::from(&cap["adj"], &cap["color"]),
            )
        })
        .collect();

    (first_bag, other_bags)
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Bag {
    adjective: String,
    color: String,
}

impl Bag {
    fn from(adjective: impl Into<String>, color: impl Into<String>) -> Bag {
        Bag {
            adjective: adjective.into(),
            color: color.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_solution_1() {
        let demo_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";
        let solution = solve_first(demo_input, &Bag::from("shiny", "gold"));
        assert_eq!(solution, 4);
    }
    #[test]
    fn seoncd_demo_solution_1() {
        let demo_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";
        let solution = solve_second(demo_input, &Bag::from("shiny", "gold"));
        assert_eq!(solution, 32);
    }
    #[test]
    fn second_demo_solution_2() {
        let demo_input = "shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.";
        let solution = solve_second(demo_input, &Bag::from("shiny", "gold"));
        assert_eq!(solution, 126);
    }
}
