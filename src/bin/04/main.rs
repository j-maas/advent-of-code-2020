use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, convert::identity, ops::RangeBounds};

fn main() {
    let input = include_str!("./input.txt");

    let first_solution = solve_presence(input);
    println!("The first solution is:\n{}", first_solution);

    let second_solution = solve_valids(input);
    println!("The second solution is:\n{}", second_solution);
}

fn solve_presence(input: &str) -> usize {
    let passports = parse(input);

    passports
        .iter()
        .filter(|passport| passport_fields_present(passport))
        .count()
}

fn solve_valids(input: &str) -> usize {
    let passports = parse(input);

    let mut count = 0;
    passports
        .iter()
        .filter(|passport| valid_fields(passport))
        .for_each(|p| {
            println!("{:#?}", p);
            count += 1
        });
    count
}

lazy_static! {
    static ref DELIMITER_REGEX: Regex = Regex::new(r"\n\s*\n").unwrap();
    static ref KEY_VALUE_REGEX: Regex =
        Regex::new(r"(?P<key>[a-zA-Z]+):(?P<value>[a-zA-Z0-9#]+)").unwrap();
    static ref HGT_REGEX: Regex = Regex::new(r"(?P<number>[0-9]+)(?P<unit>cm|in)$").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"#[0-9a-f]{6}$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"[0-9]{9}").unwrap();
    static ref VALID_ECLS: Vec<&'static str> =
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    static ref REQUIRED_KEYS: Vec<&'static str> =
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
}

fn parse(input: &str) -> Vec<Passport> {
    DELIMITER_REGEX
        .split(input)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            KEY_VALUE_REGEX
                .captures_iter(line)
                .map(|cap| (cap["key"].to_string(), cap["value"].to_string()))
                .collect::<Passport>()
        })
        .collect()
}

fn passport_fields_present(passport: &Passport) -> bool {
    REQUIRED_KEYS
        .iter()
        .all(|required| passport.contains_key(*required))
}

fn valid_fields(passport: &Passport) -> bool {
    passport
        .get("byr")
        .map(|field| number_in_range(field, &(1920..=2002)))
        .unwrap_or(false)
        && passport
            .get("iyr")
            .map(|field| number_in_range(field, &(2010..=2020)))
            .unwrap_or(false)
        && passport
            .get("eyr")
            .map(|field| number_in_range(field, &(2020..=2030)))
            .unwrap_or(false)
        && passport
            .get("hgt")
            .map(|field| valid_hgt(field))
            .unwrap_or(false)
        && passport
            .get("hcl")
            .map(|field| valid_hcl(field))
            .unwrap_or(false)
        && passport
            .get("ecl")
            .map(|field| valid_ecl(field))
            .unwrap_or(false)
        && passport
            .get("pid")
            .map(|field| valid_pid(field))
            .unwrap_or(false)
}

fn number_in_range<R>(field: &str, range: &R) -> bool
where
    R: RangeBounds<usize>,
{
    field
        .parse::<usize>()
        .map(|number| range.contains(&number))
        .unwrap_or(false)
}

fn valid_hgt(hgt: &str) -> bool {
    HGT_REGEX
        .captures(hgt)
        .map(|cap| {
            let range = if &cap["unit"] == "cm" {
                150..=193
            } else {
                59..=76
            };

            cap["number"]
                .parse::<usize>()
                .map(|number| range.contains(&number))
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

fn valid_hcl(hcl: &str) -> bool {
    HCL_REGEX.is_match(hcl)
}

fn valid_ecl(ecl: &str) -> bool {
    VALID_ECLS.iter().any(|valid| &ecl == valid)
}

fn valid_pid(pid: &str) -> bool {
    PID_REGEX.is_match(pid)
}

type Passport = HashMap<String, String>;

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO_INPUT: &str = "
    ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm
    
    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929
    
    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm
    
    hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn demo_solution_1() {
        let solution = solve_presence(DEMO_INPUT);
        assert_eq!(solution, 2);
    }

    #[test]
    fn demo_solution_valids() {
        let valid_inputs = "
        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        ";

        let solution = solve_valids(valid_inputs);
        assert_eq!(solution, 4);
    }

    #[test]
    fn demo_solution_invalids() {
        let invalid_inputs = "
        eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007

        byr:2001
        iyr:2015
        eyr:2022
        hgt:180cma
        hcl:#888785
        ecl:hzl
        pid:545766238
        cid:88
        ";

        let solution = solve_valids(invalid_inputs);
        assert_eq!(solution, 0);
    }
}
