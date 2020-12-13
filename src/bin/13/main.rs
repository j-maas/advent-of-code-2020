use modinverse::modinverse;

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> usize {
    let (arrival_time, busses) = parse(input);
    let mut time = arrival_time;

    loop {
        let maybe_bus = busses.iter().find(|(_, bus)| time % *bus == 0);
        if let Some((_, bus)) = maybe_bus {
            return *bus * (time - arrival_time);
        }

        time += 1;
    }
}

fn solve_second(input: &str) -> i128 {
    let (_, busses) = parse(input);

    let n: i128 = busses.iter().map(|(_, bus)| *bus as i128).product();
    let result: i128 = busses
        .iter()
        .map(|(index_, bus_)| {
            let index = *index_ as i128;
            let bus = *bus_ as i128;

            let a = (bus - index) % bus;
            let y = n / bus;
            let z = modinverse(y as i128, bus as i128).unwrap();

            a * y * (z as i128)
        })
        .sum();
    result % n
}

fn parse(input: &str) -> (usize, Vec<(usize, usize)>) {
    let mut lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let time = lines.next().unwrap().parse().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(index, raw_bus)| match raw_bus.parse() {
            Ok(bus) => Some((index, bus)),
            Err(_) => None,
        })
        .collect();

    (time, busses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_demo_solution() {
        let demo_input = "939
        7,13,x,x,59,x,31,19";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 59 * 5);
    }

    #[test]
    fn second_demo_solution_1() {
        let demo_input = "939
        7,13,x,x,59,x,31,19";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 1068781);
    }
    #[test]
    fn second_demo_solution_2() {
        let demo_input = "939
        17,x,13,19";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 3417);
    }
    #[test]
    fn second_demo_solution_3() {
        let demo_input = "939
        67,7,59,61";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 754018);
    }
    #[test]
    fn second_demo_solution_4() {
        let demo_input = "939
        67,x,7,59,61";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 779210);
    }
    #[test]
    fn second_demo_solution_5() {
        let demo_input = "939
        67,7,x,59,61";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 1261476);
    }
    #[test]
    fn second_demo_solution_6() {
        let demo_input = "939
        1789,37,47,1889";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 1202161486);
    }
    #[test]
    fn second_demo_solution_7() {
        let demo_input = "939
        2,3,5";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 8);
    }
    #[test]
    fn second_demo_solution_8() {
        let demo_input = "939
        3,2,5";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 3);
    }
}
