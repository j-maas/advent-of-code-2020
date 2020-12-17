use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = second::solve(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> usize {
    let mut board = parse(input);

    for _ in 0..6 {
        board = step(board);
    }

    board.len()
}

fn parse(input: &str) -> Board {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, cols)| {
            cols.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as isize, y as isize, 0))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn step(board: Board) -> Board {
    let mut active_neighbor_counts: HashMap<Coords, usize> = HashMap::new();
    let mut new_board: HashSet<Coords> = HashSet::new();

    for coord in board.iter() {
        let neighbors = neighbors(coord);

        let active_neighbors = neighbors.iter().filter(|c| board.contains(c)).count();
        if (2..=3).contains(&active_neighbors) {
            new_board.insert(*coord);
        }

        for neighbor in neighbors.iter() {
            active_neighbor_counts
                .entry(*neighbor)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    let new_active: HashSet<Coords> = active_neighbor_counts
        .iter()
        .filter(|(_, count)| **count == 3)
        .map(|(coord, _)| *coord)
        .collect();

    new_board.union(&new_active).cloned().collect()
}

fn neighbors(coords: &Coords) -> [Coords; 26] {
    [-1, -1, -1, 0, 0, 0, 1, 1, 1]
        .iter()
        .permutations(3)
        .map(|v| (*v[0], *v[1], *v[2]))
        .unique()
        .filter(|c| c != &(0, 0, 0))
        .map(|c| (c.0 + coords.0, c.1 + coords.1, c.2 + coords.2))
        .collect::<Vec<Coords>>()
        .try_into()
        .unwrap()
}

type Board = HashSet<Coords>;

type Coords = (isize, isize, isize);

mod second {
    use itertools::Itertools;
    use lazy_static::lazy_static;
    use std::{
        collections::{HashMap, HashSet},
        convert::TryInto,
    };

    pub fn solve(input: &str) -> usize {
        let mut board = parse(input);

        for _ in 0..6 {
            board = step(board);
        }

        board.len()
    }

    fn parse(input: &str) -> Board {
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .enumerate()
            .flat_map(|(y, cols)| {
                cols.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((x as isize, y as isize, 0, 0))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn step(board: Board) -> Board {
        let mut active_neighbor_counts: HashMap<Coords, usize> = HashMap::new();
        let mut new_board: HashSet<Coords> = HashSet::new();

        for coord in board.iter() {
            let neighbors = neighbors(coord);

            let active_neighbors = neighbors.iter().filter(|c| board.contains(c)).count();
            if (2..=3).contains(&active_neighbors) {
                new_board.insert(*coord);
            }

            for neighbor in neighbors.iter() {
                active_neighbor_counts
                    .entry(*neighbor)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        let new_active: HashSet<Coords> = active_neighbor_counts
            .iter()
            .filter(|(_, count)| **count == 3)
            .map(|(coord, _)| *coord)
            .collect();

        new_board.union(&new_active).cloned().collect()
    }

    lazy_static! {
        static ref NEIGHBOR_DIFFS: Vec<Coords> = [-1, -1, -1, -1, 0, 0, 0, 0, 1, 1, 1, 1]
            .iter()
            .permutations(4)
            .map(|v| (*v[0], *v[1], *v[2], *v[3]))
            .unique()
            .filter(|c| c != &(0, 0, 0, 0))
            .collect::<Vec<Coords>>();
    }

    fn neighbors(coords: &Coords) -> [Coords; 80] {
        NEIGHBOR_DIFFS
            .iter()
            .map(|c| {
                (
                    c.0 + coords.0,
                    c.1 + coords.1,
                    c.2 + coords.2,
                    c.3 + coords.3,
                )
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    type Board = HashSet<Coords>;

    type Coords = (isize, isize, isize, isize);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_demo_solution_1() {
        let demo_input = ".#.
        ..#
        ###";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 112);
    }

    #[test]
    fn second_demo_solution_1() {
        let demo_input = ".#.
        ..#
        ###";
        let solution = second::solve(demo_input);
        assert_eq!(solution, 848);
    }
}
