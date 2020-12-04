use crate::utils::read_input_to_string;
use regex::Regex;
use std::collections::HashMap;

const REQUIRED_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

type Passport = HashMap<String, String>;

pub fn day4() {
    let passports = read_input();
    println!("DAY 4");
    println!(
        "There are {} valid passports.",
        how_many_valid(&passports, contains_required_fields)
    );
    println!(
        "Actually, there are {} valid passports.",
        how_many_valid(&passports, |p| contains_required_fields(p)
            && fields_are_valid(p))
    );
}

fn how_many_valid<F: Fn(&Passport) -> bool>(passports: &Vec<Passport>, is_valid: F) -> usize {
    passports.iter().filter(|p| is_valid(&p)).count()
}

fn contains_required_fields(passport: &Passport) -> bool {
    REQUIRED_FIELDS.iter().all(|&k| passport.contains_key(k))
}

fn fields_are_valid(passport: &Passport) -> bool {
    lazy_static! {
        static ref HCL_RE: Regex = Regex::new(r"#([0-9]|[a-f]){6}").unwrap();
    }

    passport.iter().all(|(k, v)| match k.as_str() {
        "byr" => nat_between(v, 1920, 2002),
        "iyr" => nat_between(v, 2010, 2020),
        "eyr" => nat_between(v, 2020, 2030),
        "hgt" => {
            let val = v[0..v.len() - 2].to_string();
            (v.ends_with("cm") && nat_between(&val, 150, 193))
                || (v.ends_with("in") && nat_between(&val, 59, 76))
        }
        "hcl" => HCL_RE.is_match(v),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str()),
        "pid" => v.len() == 9 && v.parse::<u32>().is_ok(),
        _ => true,
    })
}

fn nat_between(s: &String, low: u32, high: u32) -> bool {
    s.parse::<u32>()
        .map_or(false, |year| low <= year && year <= high)
}

fn read_input() -> Vec<Passport> {
    read_input_to_string(String::from("day4"))
        .split("\n\n")
        .map(parse_passport)
        .collect()
}

fn parse_passport(passport: &str) -> HashMap<String, String> {
    passport
        .split_whitespace()
        .map(|kv| kv.split(':').collect::<Vec<&str>>())
        .map(|kv| (kv[0].to_string(), kv[1].to_string()))
        .collect()
}
