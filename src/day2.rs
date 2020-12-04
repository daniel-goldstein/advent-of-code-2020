use crate::utils::read_input_to_string;
use regex::Regex;

type Password = (String, char, usize, usize);

pub fn day2() {
    println!("DAY 2");
    println!(
        "There are {} valid passwords",
        how_many_valid(read_input(), is_valid_occurences)
    );
    println!(
        "Actually, there are {} valid passwords",
        how_many_valid(read_input(), is_valid_indices)
    );
}

fn how_many_valid<F: Fn(&Password) -> bool>(passwords: Vec<Password>, is_valid: F) -> usize {
    passwords.into_iter().filter(is_valid).count()
}

fn is_valid_occurences(password_and_policy: &Password) -> bool {
    let (password, letter, at_least, at_most) = password_and_policy;
    let count = password.chars().filter(|c| c == letter).count();
    at_least <= &count && &count <= at_most
}

fn is_valid_indices(password_and_policy: &Password) -> bool {
    let (password, letter, first, second) = password_and_policy;
    let at_first = password.chars().nth(*first - 1).unwrap() == *letter;
    let at_second = password.chars().nth(*second - 1).unwrap() == *letter;

    (at_first || at_second) && (at_first != at_second)
}

fn read_input() -> Vec<Password> {
    let contents = read_input_to_string(String::from("day2"));

    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    re.captures_iter(contents.as_str())
        .map(|cap| {
            let first_num = cap[1].parse::<usize>().unwrap();
            let second_num = cap[2].parse::<usize>().unwrap();
            let letter = cap[3].chars().nth(0).unwrap();
            let password = cap[4].to_string();

            (password, letter, first_num, second_num)
        })
        .collect::<Vec<Password>>()
}
