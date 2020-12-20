use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::convert::TryInto;

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_first(input);
    println!("The first solution is:\n{}", first_solution);

    /*let second_solution = solve_second(input);
    println!("The second solution is:\n{}", second_solution);
    */
}

fn solve_first(input: &str) -> usize {
    let images = parse(input);
    //println!("{:#?}", images);

    let mut map: HashMap<usize, Vec<Id>> = HashMap::new();
    for (id, borders) in images {
        for border in borders.iter() {
            map.entry(*border)
                .and_modify(|neighbors| neighbors.push(id))
                .or_insert_with(|| vec![id]);
        }
    }
    //println!("{:#?}", map);
    let mut connected: HashMap<Id, Vec<Id>> = HashMap::new();
    for (_, ids) in map {
        if ids.len() == 2 {
            let first = ids[0];
            let second = ids[1];
            connected
                .entry(first)
                .and_modify(|neighbors| neighbors.push(second))
                .or_insert_with(|| vec![second]);
            connected
                .entry(second)
                .and_modify(|neighbors| neighbors.push(first))
                .or_insert_with(|| vec![first]);
        }
    }
    //println!("{:#?}", connected);

    let corners: Vec<Id> = connected
        .into_iter()
        .filter(|(_, neighbors)| neighbors.len() == 2)
        .map(|(id, _)| id)
        .collect();

    //println!("{:?}", corners);
    assert_eq!(corners.len(), 4);
    corners.iter().product()
}

fn parse(input: &str) -> Vec<(Id, [usize; 4])> {
    let parts = BLANK_LINE_REGEX.split(input);

    parts
        .map(|image| {
            let mut lines = image.lines();
            //println!("{}", image);
            let id_line = lines.next().unwrap().trim();
            let id = id_line[5..(id_line.len() - 1)].parse().unwrap();
            let pixels: Vec<Vec<char>> = lines
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .map(|line| line.chars().collect())
                .collect();
            let transposed: Vec<Vec<char>> = (0..pixels[0].len())
                .map(|index| {
                    pixels
                        .iter()
                        .map(|numbers| numbers[index].clone())
                        .collect::<Vec<_>>()
                })
                .collect();
            let borders = [
                pixels[0].clone(),
                transposed[0].clone(),
                pixels[pixels.len() - 1].clone(),
                transposed[transposed.len() - 1].clone(),
            ]
            .iter()
            .map(|border| {
                let border_number = border
                    .iter()
                    .enumerate()
                    .fold(0usize, |acc, (index, next)| {
                        let digit = match next {
                            '#' => 1,
                            '.' => 0,
                            _ => unreachable!(),
                        };
                        acc + (2usize.pow(index.try_into().unwrap()) * digit)
                    });
                let negated = border
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0usize, |acc, (index, next)| {
                        let digit = match next {
                            '#' => 1,
                            '.' => 0,
                            _ => unreachable!(),
                        };
                        acc + (2usize.pow(index.try_into().unwrap()) * digit)
                    });
                //println!("{}: {} vs {}", id, border_number, negated);
                std::cmp::min(border_number, negated)
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
            (id, borders)
        })
        .collect()
}

lazy_static! {
    static ref BLANK_LINE_REGEX: Regex = Regex::new(r"\n\s*\n").unwrap();
    static ref RULE_ID_REGEX: Regex = Regex::new(r"(?P<id>\d+): (?P<rule>.+)$").unwrap();
    static ref LITERAL_RULE_REGEX: Regex = Regex::new(r#""(?P<literal>.)""#).unwrap();
}

type Id = usize;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_demo_solution_1() {
        let demo_input = "Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###
        
        Tile 1951:
        #.##...##.
        #.####...#
        .....#..##
        #...######
        .##.#....#
        .###.#####
        ###.##.##.
        .###....#.
        ..#.#..#.#
        #...##.#..
        
        Tile 1171:
        ####...##.
        #..##.#..#
        ##.#..#.#.
        .###.####.
        ..###.####
        .##....##.
        .#...####.
        #.##.####.
        ####..#...
        .....##...
        
        Tile 1427:
        ###.##.#..
        .#..#.##..
        .#.##.#..#
        #.#.#.##.#
        ....#...##
        ...##..##.
        ...#.#####
        .#.####.#.
        ..#..###.#
        ..##.#..#.
        
        Tile 1489:
        ##.#.#....
        ..##...#..
        .##..##...
        ..#...#...
        #####...#.
        #..#.#.#.#
        ...#.#.#..
        ##.#...##.
        ..##.##.##
        ###.##.#..
        
        Tile 2473:
        #....####.
        #..#.##...
        #.##..#...
        ######.#.#
        .#...#.#.#
        .#########
        .###.#..#.
        ########.#
        ##...##.#.
        ..###.#.#.
        
        Tile 2971:
        ..#.#....#
        #...###...
        #.#.###...
        ##.##..#..
        .#####..##
        .#..####.#
        #..#.#..#.
        ..####.###
        ..#.#.###.
        ...#.#.#.#
        
        Tile 2729:
        ...#.#.#.#
        ####.#....
        ..#.#.....
        ....#..#.#
        .##..##.#.
        .#.####...
        ####.#.#..
        ##.####...
        ##..#.##..
        #.##...##.
        
        Tile 3079:
        #.#.#####.
        .#..######
        ..#.......
        ######....
        ####.#..#.
        .#...#.##.
        #.#####.##
        ..#.###...
        ..#.......
        ..#.###...";
        let solution = solve_first(demo_input);
        assert_eq!(solution, 20899048083289);
    }
}
