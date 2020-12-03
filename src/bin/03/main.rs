fn main() {
    let input = include_str!("./input.txt");
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let first_solution = solve(input, 3, 1);
    println!("The first solution is:\n{}", first_solution);

    let second_solution: usize = slopes
        .into_iter()
        .map(|(right, down)| solve(input, right, down))
        .product();

    println!("The second solution is:\n{}", second_solution);
}

fn solve(input: &str, right: usize, down: usize) -> usize {
    let map = parse(input);
    let height = map.len();
    let width = map[0].len();

    let mut row = 0;
    let mut column = 0;
    let mut trees = 0;
    while row < height {
        let field = map[row].chars().nth(column).unwrap();
        if field == '#' {
            trees += 1;
        }

        column = (column + right) % width;
        row += down;
    }

    trees
}

fn parse(input: &str) -> Vec<&str> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO_INPUT: &str = "..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#";

    #[test]
    fn demo_solution_1() {
        let solution = solve(DEMO_INPUT, 1, 1);
        assert_eq!(solution, 2);
    }
    #[test]
    fn demo_solution_2() {
        let solution = solve(DEMO_INPUT, 3, 1);
        assert_eq!(solution, 7);
    }
    #[test]
    fn demo_solution_3() {
        let solution = solve(DEMO_INPUT, 5, 1);
        assert_eq!(solution, 3);
    }
    #[test]
    fn demo_solution_4() {
        let solution = solve(DEMO_INPUT, 7, 1);
        assert_eq!(solution, 4);
    }

    #[test]
    fn demo_solution_5() {
        let solution = solve(DEMO_INPUT, 1, 2);
        assert_eq!(solution, 2);
    }
}
