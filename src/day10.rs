use std::collections::HashMap;
use std::{cmp::Ordering, collections::HashSet, iter::FromIterator};

use aoc_runner_derive::{aoc, aoc_generator};

fn diff_dist(input: &[usize]) -> (usize, usize, usize) {
    let mut diff = (0, 0, 0);
    let mut jolts = input.to_vec();
    jolts.sort();
    for (&x, &y) in jolts.iter().zip(jolts[1..].iter()) {
        match y - x {
            1 => diff.0 += 1,
            2 => diff.1 += 1,
            3 => diff.2 += 1,
            _ => panic!(format!("{} and {} have invalid difference.", x, y)),
        };
    }
    diff
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

#[aoc(day10, part1)]
fn adaptor_chain(input: &[usize]) -> usize {
    let mut adaptors = input.to_vec();
    adaptors.push(0);
    let (a, _b, c) = diff_dist(&adaptors);
    return a * (c + 1);
}

fn check_option(
    cur: usize,
    options: &HashSet<usize>,
    visited: &mut HashMap<usize, usize>,
    goal: usize,
) -> usize {
    match cur.cmp(&goal) {
        Ordering::Greater => 0,
        Ordering::Equal => 1,
        Ordering::Less => {
            if let Some(&value) = visited.get(&cur) {
                return value;
            }
            let value = (cur + 1..cur + 4)
                .rev()
                .filter(|o| options.contains(o))
                .map(|o| check_option(o, options, visited, goal))
                .sum();
            visited.insert(cur, value);
            value
        }
    }
}

#[aoc(day10, part2)]
fn adaptor_options(input: &[usize]) -> usize {
    let options = HashSet::from_iter(input.iter().cloned());
    let max = options.iter().max().unwrap();
    let mut paths = HashMap::new();

    check_option(0, &options, &mut paths, *max)
}

#[cfg(test)]
mod tests {
    use super::{adaptor_options, diff_dist};

    #[test]
    fn test_one() {
        let actual = diff_dist(&[0usize, 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]);
        let expected: (usize, usize, usize) = (7, 0, 4);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_two() {
        let actual = diff_dist(&[
            0usize, 28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32,
            25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
        ]);
        let expected: (usize, usize, usize) = (22, 0, 9);
        assert_eq!(actual, expected);
    }

    #[test]
    fn options() {
        assert_eq!(adaptor_options(&[1usize, 2, 3, 4]), 7);
        assert_eq!(adaptor_options(&[1usize, 2, 4, 6, 7]), 6);
        assert_eq!(adaptor_options(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]), 8);
        assert_eq!(
            adaptor_options(&[
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
            ]),
            19208
        );
    }
}
