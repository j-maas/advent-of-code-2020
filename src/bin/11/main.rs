use std::{
    convert::{TryFrom, TryInto},
    fmt::{Debug, Display},
};

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_first(input: &str) -> usize {
    let mut board = parse(input);
    let mut next_board = board.clone();
    let mut changed;

    loop {
        changed = false;
        next_board = next_board
            .iter()
            .enumerate()
            .map(|(row, cells)| {
                cells
                    .iter()
                    .enumerate()
                    .map(|(col, cell)| {
                        use Cell::*;
                        match cell {
                            Seat(false) => {
                                let occupied_neighbors = neighbors(row, col, &board)
                                    .iter()
                                    .filter(|c| c == &&Seat(true))
                                    .count();
                                if occupied_neighbors == 0 {
                                    changed = true;
                                    Seat(true)
                                } else {
                                    Seat(false)
                                }
                            }
                            Seat(true) => {
                                let occupied_neighbors = neighbors(row, col, &board)
                                    .iter()
                                    .filter(|c| c == &&Seat(true))
                                    .count();
                                if occupied_neighbors >= 4 {
                                    changed = true;
                                    Seat(false)
                                } else {
                                    Seat(true)
                                }
                            }
                            cell => *cell,
                        }
                    })
                    .collect()
            })
            .collect();

        board = next_board.clone();

        if !changed {
            break;
        }
    }

    board
        .iter()
        .flatten()
        .filter(|c| c == &&Cell::Seat(true))
        .count()
}

fn solve_second(input: &str) -> usize {
    let mut board = parse(input);
    let mut next_board = board.clone();
    let mut changed;

    loop {
        changed = false;
        //println!("{}\n\n", display_board(&board));

        next_board = next_board
            .iter()
            .enumerate()
            .map(|(row, cells)| {
                cells
                    .iter()
                    .enumerate()
                    .map(|(col, cell)| {
                        use Cell::*;
                        match cell {
                            Seat(false) => {
                                let occupied_neighbors = visible_neighbors(row, col, &board)
                                    .iter()
                                    .filter(|c| c == &&Seat(true))
                                    .count();
                                if occupied_neighbors == 0 {
                                    changed = true;
                                    Seat(true)
                                } else {
                                    Seat(false)
                                }
                            }
                            Seat(true) => {
                                let occupied_neighbors = visible_neighbors(row, col, &board)
                                    .iter()
                                    .filter(|c| c == &&Seat(true))
                                    .count();
                                if occupied_neighbors >= 5 {
                                    changed = true;
                                    Seat(false)
                                } else {
                                    Seat(true)
                                }
                            }
                            cell => *cell,
                        }
                    })
                    .collect()
            })
            .collect();

        board = next_board.clone();

        if !changed {
            break;
        }
    }

    board
        .iter()
        .flatten()
        .filter(|c| c == &&Cell::Seat(true))
        .count()
}

fn parse(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.try_into().unwrap()).collect())
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Cell {
    Floor,
    Seat(bool),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        f.write_str(match self {
            Floor => ".",
            Seat(true) => "#",
            Seat(false) => "L",
        })
    }
}

fn display_board(board: &Vec<Vec<Cell>>) -> String {
    board
        .iter()
        .map(|cells| {
            cells
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn neighbors(row: usize, col: usize, board: &Vec<Vec<Cell>>) -> Vec<Cell> {
    let up = row.checked_sub(1);
    let down = {
        let d = row + 1;
        if d >= board.len() {
            None
        } else {
            Some(d)
        }
    };
    let left = col.checked_sub(1);
    let right = {
        let r = col + 1;
        if r >= board[0].len() {
            None
        } else {
            Some(r)
        }
    };

    let mut neighbors = vec![];

    if let (Some(r), Some(c)) = (up, left) {
        neighbors.push(board[r][c]);
    }
    if let (Some(r), Some(c)) = (up, Some(col)) {
        neighbors.push(board[r][c]);
    }
    if let (Some(r), Some(c)) = (up, right) {
        neighbors.push(board[r][c]);
    }
    if let (Some(r), Some(c)) = (Some(row), left) {
        neighbors.push(board[r][c]);
    }
    if let (Some(r), Some(c)) = (Some(row), right) {
        neighbors.push(board[r][c]);
    }
    if let (Some(r), Some(c)) = (down, left) {
        neighbors.push(board[r][c]);
    }
    if let (Some(r), Some(c)) = (down, Some(col)) {
        neighbors.push(board[r][c]);
    }
    if let (Some(r), Some(c)) = (down, right) {
        neighbors.push(board[r][c]);
    }

    neighbors
}

fn visible_neighbors(row: usize, col: usize, board: &Vec<Vec<Cell>>) -> Vec<Cell> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter_map(|(row_inc, col_inc): &(isize, isize)| {
        let mut next_row = row as isize + row_inc;
        let mut next_col = col as isize + col_inc;

        while 0 <= next_row
            && next_row < board.len() as isize
            && 0 <= next_col
            && next_col < board[0].len() as isize
        {
            let cell = board[next_row as usize][next_col as usize];
            use Cell::*;
            if let Seat(occupied) = cell {
                return Some(Seat(occupied));
            }

            next_row += row_inc;
            next_col += col_inc;
        }

        None
    })
    .collect()
}

impl TryFrom<char> for Cell {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Cell::*;
        match value {
            '.' => Ok(Floor),
            'L' => Ok(Seat(false)),
            '#' => Ok(Seat(true)),
            invalid => Err(invalid),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_demo_solution() {
        let demo_input = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 37);
    }

    #[test]
    fn second_demo_solution() {
        let demo_input = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let solution = solve_second(demo_input);
        assert_eq!(solution, 26);
    }
}
