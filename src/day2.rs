use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Password> {
    let pattern: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    pattern
        .captures_iter(input)
        .map(|entry| Password {
            min: entry.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            max: entry.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            character: entry.get(3).unwrap().as_str().chars().next().unwrap(),
            password: String::from(entry.get(4).unwrap().as_str()),
        })
        .collect()
}

#[aoc(day2, part1)]
fn valid_passwords_count(input: &Vec<Password>) -> usize {
    input
        .iter()
        .filter(|entry| {
            let count = entry.password.matches(entry.character).count();
            entry.min <= count && entry.max >= count
        })
        .count()
}

#[aoc(day2, part2)]
fn valid_passwords_position(input: &Vec<Password>) -> usize {
    input
        .iter()
        .filter(|entry| {
            entry
                .password
                .chars()
                .enumerate()
                .filter(|(i, ch)| {
                    *ch == entry.character && (*i == entry.min - 1 || *i == entry.max - 1)
                })
                .count()
                == 1
        })
        .count()
}
