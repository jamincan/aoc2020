use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Group {
    count: usize,
    intersection: HashSet<char>,
    union: HashSet<char>,
}

impl Group {
    fn new() -> Group {
        Group {
            count: 0,
            intersection: HashSet::new(),
            union: HashSet::new(),
        }
    }
    fn add(&mut self, question: &str) {
        for ch in question.chars() {
            self.union.insert(ch);
            if self.count == 0 {
                self.intersection.insert(ch);
            }
        }
        if self.count > 0 {
            self.intersection = self
                .intersection
                .intersection(&question.chars().collect())
                .map(|&c| c)
                .collect()
        }
        self.count += 1;
    }
}

impl From<&str> for Group {
    fn from(input: &str) -> Self {
        let mut group = Group::new();
        let members = input.lines();
        for member in members {
            group.add(member);
        }
        group
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<Group> {
    let pattern = Regex::new(r"(\r\n|\n|\r){2,}").unwrap();
    pattern.split(input).map(|g| Group::from(g)).collect()
}

#[aoc(day6, part1)]
fn count_union(input: &Vec<Group>) -> usize {
    let mut count = 0;
    for grp in input {
        count += grp.union.len();
    }
    count
}

#[aoc(day6, part2)]
fn count_intersection(input: &Vec<Group>) -> usize {
    let mut count = 0;
    for grp in input {
        count += grp.intersection.len();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn group_no_intersection() {
        let expected = Group {
            count: 3,
            union: "abcd".chars().collect(),
            intersection: "".chars().collect(),
        };
        let mut actual = Group::new();
        actual.add("ab");
        actual.add("bc");
        actual.add("cd");
        assert_eq!(expected, actual);
    }
    #[test]
    fn group_intersection() {
        let expected = Group {
            count: 3,
            union: "abcdef".chars().collect(),
            intersection: "ab".chars().collect(),
        };
        let mut actual = Group::new();
        actual.add("abc");
        actual.add("abde");
        actual.add("abf");
        assert_eq!(expected, actual);
    }
}
