use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> usize {
    let (rules, messages) = parse(input);
    let regex = rules_to_regex(&rules);

    messages
        .iter()
        .filter(|message| regex.is_match(message))
        .count()
}

fn solve_second(input: &str) -> usize {
    let (mut rules, messages) = parse(input);

    rules.insert(8, Rule::Pattern(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::Pattern(vec![vec![42, 31], vec![42, 11, 31]]));

    messages
        .iter()
        .enumerate()
        //.inspect(|(index, message)| println!("{} ===================== {}", index, message))
        .filter(|(_, message)| matches(message, &rules))
        //.inspect(|(index, message)| println!("{} {}", index, message))
        .count()
}

fn parse(input: &str) -> (Rules, Vec<String>) {
    let mut parts = BLANK_LINE_REGEX.split(input);

    let rules = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| parse_rule(line))
        .collect();

    let messages = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    (rules, messages)
}

lazy_static! {
    static ref BLANK_LINE_REGEX: Regex = Regex::new(r"\n\s*\n").unwrap();
    static ref RULE_ID_REGEX: Regex = Regex::new(r"(?P<id>\d+): (?P<rule>.+)$").unwrap();
    static ref LITERAL_RULE_REGEX: Regex = Regex::new(r#""(?P<literal>.)""#).unwrap();
}

fn parse_rule(input: &str) -> (RuleId, Rule) {
    let cap = RULE_ID_REGEX.captures(input).unwrap();
    let id = cap["id"].parse().unwrap();
    let raw_rule = &cap["rule"];
    let rule = if let Some(literal_cap) = LITERAL_RULE_REGEX.captures(raw_rule) {
        Rule::Literal(literal_cap["literal"].chars().next().unwrap())
    } else {
        Rule::Pattern(
            raw_rule
                .split(" | ")
                .map(|sequence| {
                    sequence
                        .split(" ")
                        .map(|raw_id| raw_id.parse().unwrap())
                        .collect()
                })
                .collect(),
        )
    };
    (id, rule)
}

#[derive(Debug)]
enum Rule {
    Pattern(Alternatives),
    Literal(char),
}

type Rules = HashMap<RuleId, Rule>;
type Alternatives = Vec<Sequence>;
type Sequence = Vec<RuleId>;
type RuleId = usize;

fn rules_to_regex(rules: &Rules) -> Regex {
    let raw_regex = format!("^{}$", &rules_to_regex_string(&0, rules));
    Regex::new(&raw_regex).unwrap()
}

fn rules_to_regex_string(rule_id: &RuleId, rules: &Rules) -> String {
    match &rules[rule_id] {
        Rule::Literal(l) => l.to_string(),
        Rule::Pattern(alternatives) => {
            format!(
                "({})",
                alternatives
                    .iter()
                    .map(|seq| {
                        seq.iter()
                            .map(|id| rules_to_regex_string(id, rules))
                            .join("")
                    })
                    .join("|")
            )
        }
    }
}

fn matches(message: &str, rules: &Rules) -> bool {
    let mut possibilities: Possibilities = VecDeque::from(vec![Path {
        candidate: "".to_string(),
        remaining: VecDeque::from(vec![0]),
    }]);

    loop {
        possibilities = next(possibilities, rules);
        if let Some(current) = possibilities.front() {
            if !message.starts_with(&current.candidate) {
                possibilities.pop_front();
            } else if current.remaining.is_empty() {
                if message == current.candidate {
                    return true;
                } else {
                    possibilities.pop_front();
                }
            }
        } else {
            return false;
        }
    }
}

fn next(mut possibilities: Possibilities, rules: &Rules) -> Possibilities {
    if let Some(mut current) = possibilities.pop_front() {
        let mut new_fronts = if let Some(next) = current.remaining.pop_front() {
            match &rules[&next] {
                Rule::Literal(l) => {
                    current.candidate.push(l.clone());
                    VecDeque::from(vec![current])
                }
                Rule::Pattern(alternatives) => alternatives
                    .iter()
                    .map(|seq| {
                        let new_remaining = seq
                            .iter()
                            .cloned()
                            .chain(current.remaining.clone())
                            .collect();
                        Path {
                            candidate: current.candidate.clone(),
                            remaining: new_remaining,
                        }
                    })
                    .collect(),
            }
        } else {
            VecDeque::from(vec![current])
        };
        new_fronts.append(&mut possibilities);
        new_fronts
    } else {
        possibilities
    }
}

type ToVisit = VecDeque<RuleId>;
struct Path {
    candidate: String,
    remaining: ToVisit,
}
type Possibilities = VecDeque<Path>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_demo_solution_1() {
        let demo_input = r#"0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"
        
        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb"#;
        let solution = solve_first(demo_input);
        assert_eq!(solution, 2);
    }

    #[test]
    fn second_demo_solution_1() {
        let demo_input = r#"42: 9 14 | 10 1
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        1: "a"
        11: 42 31
        5: 1 14 | 15 1
        19: 14 1 | 14 14
        12: 24 14 | 19 1
        16: 15 1 | 14 14
        31: 14 17 | 1 13
        6: 14 14 | 1 14
        2: 1 24 | 14 4
        0: 8 11
        13: 14 3 | 1 12
        15: 1 | 14
        17: 14 2 | 1 7
        23: 25 1 | 22 14
        28: 16 1
        4: 1 1
        20: 14 14 | 1 15
        3: 5 14 | 16 1
        27: 1 6 | 14 18
        14: "b"
        21: 14 1 | 1 14
        25: 1 1 | 1 14
        22: 14 14
        8: 42
        26: 14 22 | 1 20
        18: 15 15
        7: 14 5 | 1 21
        24: 14 1
        
        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
        bbabbbbaabaabba
        babbbbaabbbbbabbbbbbaabaaabaaa
        aaabbbbbbaaaabaababaabababbabaaabbababababaaa
        bbbbbbbaaaabbbbaaabbabaaa
        bbbababbbbaaaaaaaabbababaaababaabab
        ababaaaaaabaaab
        ababaaaaabbbaba
        baabbaaaabbaaaababbaababb
        abbbbabbbbaaaababbbbbbaaaababb
        aaaaabbaabaaaaababaa
        aaaabbaaaabbaaa
        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
        babaaabbbaaabaababbaabababaaab
        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
        "#;
        let solution = solve_second(demo_input);
        assert_eq!(solution, 12);
    }

    #[test]
    fn second_demo_solution_2() {
        let demo_input = r#"0: 1
        1: "a"
        
        a
        aa
        b
        "#;
        let solution = solve_second(demo_input);
        assert_eq!(solution, 1);
    }

    #[test]
    fn second_demo_solution_3() {
        let demo_input = r#"0: 1 | 1 2
        1: "a"
        2: "b"
        
        a
        aa
        ab
        "#;
        let solution = solve_second(demo_input);
        assert_eq!(solution, 2);
    }
    #[test]
    fn second_demo_solution_4() {
        let demo_input = r#"0: 1 | 5
        1: "a"
        2: "b"
        3: 1 2 | 2 1
        4: 1
        5: 3 | 3 4
        
        baa
        "#;
        let solution = solve_second(demo_input);
        assert_eq!(solution, 1);
    }
}
