use std::collections::HashMap;

use petgraph::{algo::toposort, graphmap::GraphMap, Directed};

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> usize {
    let mut numbers = parse(input);
    numbers.insert(0, 0);

    numbers.sort_unstable();

    let differences = numbers.windows(2).map(|window| window[1] - window[0]);

    let (one, others): (Vec<usize>, Vec<usize>) =
        differences.partition(|difference| difference == &1);
    let (three, rest): (Vec<usize>, Vec<usize>) =
        others.into_iter().partition(|difference| difference == &3);

    if !rest.iter().all(|difference| difference <= &3) {
        panic!("Too much difference.");
    } else {
        return one.len() * (three.len() + 1); // Built-in is always 3 higher.
    }
}

fn solve_second(input: &str) -> usize {
    let mut numbers = parse(input);
    numbers.insert(0, 0);
    let max = numbers.iter().max().unwrap();
    let end = max + 3;
    numbers.push(end);

    let graph = make_graph(numbers);
    let topological = toposort(&graph, None).unwrap();

    let mut weights = HashMap::new();
    weights.insert(end, 1);
    // Skip the end, since we already added it.
    for node in topological.iter().rev().skip(1) {
        let neighbor_counts = graph.neighbors(*node).map(|n| weights.get(&n).unwrap());
        let weight = neighbor_counts.sum();
        weights.insert(*node, weight);
    }

    *weights.get(&0).unwrap()
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

fn make_graph(mut numbers: Vec<usize>) -> GraphMap<usize, usize, Directed> {
    let mut graph = GraphMap::new();

    numbers.sort_unstable();

    numbers.iter().for_each(|weight| {
        graph.add_node(*weight);

        &[1, 2, 3].iter().for_each(|rating| {
            let maybe_input_weight = weight.checked_sub(*rating);
            if let Some(input_weight) = maybe_input_weight {
                if graph.contains_node(input_weight) {
                    graph.add_edge(input_weight, *weight, *rating);
                }
            }
        });
    });

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_demo_solution_1() {
        let demo_input = "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 7 * 5);
    }
    #[test]
    fn first_demo_solution_2() {
        let demo_input = "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 22 * 10);
    }

    #[test]
    fn second_demo_solution_1() {
        let demo_input = "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 8);
    }
    #[test]
    fn second_demo_solution_2() {
        let demo_input = "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 19208);
    }
}
