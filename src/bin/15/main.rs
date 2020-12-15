use std::collections::HashMap;

fn main() {
    let input = "0,8,15,2,12,1,4";

    let first_solution = solve(input, 2020);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve(input, 30000000);
    println!("The second solution is:\n{}", second_solution);
}

fn solve(input: &str, end: usize) -> usize {
    let starts = parse(input);
    let mut last_seen: HashMap<usize, usize> = starts
        .iter()
        .enumerate()
        .map(|(index, number)| (*number, index))
        .take(starts.len() - 1)
        .collect();

    //println!("{:?}", last_seen);
    let mut last: usize = *starts.last().unwrap();
    for turn in (starts.len() - 1)..(end - 1) {
        let next = match last_seen.get(&last) {
            Some(last_seen_turn) => {
                /*println!(
                    "{}: Seen {} {} turns ago",
                    turn,
                    last,
                    turn - last_seen_turn
                );*/
                turn - last_seen_turn
            }
            None => {
                //println!("{}: Not yet seen {}", turn, last);
                0
            }
        };
        last_seen.insert(last, turn);
        last = next;
    }

    last
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|n| n.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_demo_solution_1() {
        let demo_input = "0,3,6";
        let solution = solve(demo_input, 2020);
        assert_eq!(solution, 436);
    }
    #[test]
    fn first_demo_solution_2() {
        let demo_input = "1,3,2";
        let solution = solve(demo_input, 2020);
        assert_eq!(solution, 1);
    }
    #[test]
    fn first_demo_solution_3() {
        let demo_input = "2,1,3";
        let solution = solve(demo_input, 2020);
        assert_eq!(solution, 10);
    }
    #[test]
    fn first_demo_solution_4() {
        let demo_input = "1,2,3";
        let solution = solve(demo_input, 2020);
        assert_eq!(solution, 27);
    }
    #[test]
    fn first_demo_solution_5() {
        let demo_input = "2,3,1";
        let solution = solve(demo_input, 2020);
        assert_eq!(solution, 78);
    }
    #[test]
    fn first_demo_solution_6() {
        let demo_input = "3,2,1";
        let solution = solve(demo_input, 2020);
        assert_eq!(solution, 438);
    }
    #[test]
    fn first_demo_solution_7() {
        let demo_input = "3,1,2";
        let solution = solve(demo_input, 2020);
        assert_eq!(solution, 1836);
    }
}
