use std::convert::{TryFrom, TryInto};

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> isize {
    let instructions = parse(input);
    let (pos, _) =
        instructions
            .iter()
            .fold(((0, 0), (1, 0)), |(pos, dir), (instruction, number)| {
                use Instruction::*;
                match instruction {
                    North => ((pos.0, pos.1 + number), dir),
                    East => ((pos.0 + number, pos.1), dir),
                    South => ((pos.0, pos.1 - number), dir),
                    West => ((pos.0 - number, pos.1), dir),
                    Forward => ((pos.0 + (dir.0 * number), pos.1 + (dir.1 * number)), dir),
                    Left => {
                        let turns = number / 90;
                        let mut new_dir = dir;
                        for _ in 0..turns {
                            new_dir = (-new_dir.1, new_dir.0)
                        }
                        (pos, new_dir)
                    }
                    Right => {
                        let turns = number / 90;
                        let mut new_dir = dir;
                        for _ in 0..turns {
                            new_dir = (new_dir.1, -new_dir.0)
                        }
                        (pos, new_dir)
                    }
                }
            });

    pos.0.abs() + pos.1.abs()
}

fn solve_second(input: &str) -> isize {
    let instructions = parse(input);
    let (pos, _): ((isize, isize), _) =
        instructions
            .iter()
            .fold(((0, 0), (10, 1)), |(pos, way), (instruction, number)| {
                use Instruction::*;
                match instruction {
                    North => (pos, (way.0, way.1 + number)),
                    East => (pos, (way.0 + number, way.1)),
                    South => (pos, (way.0, way.1 - number)),
                    West => (pos, (way.0 - number, way.1)),
                    Forward => ((pos.0 + (way.0 * number), pos.1 + (way.1 * number)), way),
                    Left => {
                        let turns = number / 90;
                        let mut new_way = way;
                        for _ in 0..turns {
                            new_way = (-new_way.1, new_way.0)
                        }
                        (pos, new_way)
                    }
                    Right => {
                        let turns = number / 90;
                        let mut new_way = way;
                        for _ in 0..turns {
                            new_way = (new_way.1, -new_way.0)
                        }
                        (pos, new_way)
                    }
                }
            });

    println!("{:?}", pos);
    pos.0.abs() + pos.1.abs()
}

fn parse(input: &str) -> Vec<(Instruction, isize)> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            (
                line.chars().nth(0).unwrap().try_into().unwrap(),
                line.chars().skip(1).collect::<String>().parse().unwrap(),
            )
        })
        .collect()
}

enum Instruction {
    North,
    East,
    South,
    West,
    Forward,
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Instruction::*;
        match value {
            'N' => Ok(North),
            'S' => Ok(South),
            'E' => Ok(East),
            'W' => Ok(West),
            'L' => Ok(Left),
            'R' => Ok(Right),
            'F' => Ok(Forward),
            c => Err(c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_demo_solution() {
        let demo_input = "F10
        N3
        F7
        R90
        F11";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 25);
    }

    #[test]
    fn test_all_directions() {
        let demo_input = "F10
        R180
        F10";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 0);
    }

    #[test]
    fn second_demo_solution() {
        let demo_input = "F10
        N3
        F7
        R90
        F11";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 286);
    }
}
