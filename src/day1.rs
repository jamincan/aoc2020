use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn find_2020_pairs(input: &Vec<usize>) -> Option<usize> {
    let mut pairs = input.iter().combinations(2);
    pairs.find_map(|pair| {
        if let [a, b] = pair[..] {
            match a + b {
                2020 => Some(a * b),
                _ => None,
            }
        } else {
            None
        }
    })
}

#[aoc(day1, part2)]
pub fn find_2020_triples(input: &Vec<usize>) -> Option<usize> {
    let mut triples = input.iter().combinations(3);
    triples.find_map(|pair| {
        if let [a, b, c] = pair[..] {
            match a + b + c {
                2020 => Some(a * b * c),
                _ => None,
            }
        } else {
            None
        }
    })
}
