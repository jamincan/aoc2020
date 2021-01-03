use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Colour(String);

impl From<&str> for Colour {
    fn from(s: &str) -> Colour {
        Colour(s.to_string())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Content {
    colour: Colour,
    count: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct ContainsRule {
    container: Colour,
    contents: Vec<Content>,
}

#[derive(Debug)]
struct RuleSet {
    rules: HashMap<Colour, Vec<Content>>,
}

impl RuleSet {
    fn contains(&self, from: &Colour, to: &Colour) -> bool {
        match self.rules.get(from) {
            Some(contents) => contents.iter().any(|c| {
                if &c.colour == to {
                    true
                } else {
                    self.contains(&c.colour, to)
                }
            }),
            _ => false,
        }
    }

    fn count_contents(&self, from: &Colour) -> usize {
        match self.rules.get(from) {
            Some(contents) => contents
                .iter()
                .map(|c| &c.count * (1 + self.count_contents(&c.colour)))
                .sum(),
            None => 0,
        }
    }
}

fn parse_rule(input: &str) -> (Colour, Vec<Content>) {
    let pattern = Regex::new(r"((\d+) )?([a-z ]+?) bag").unwrap();
    let mut caps = pattern.captures_iter(input);
    let colour = caps.next().unwrap().get(3).unwrap().as_str();
    let mut contents = Vec::new();
    for cap in caps {
        if let Some(num) = cap.get(2) {
            let count = num.as_str().parse::<usize>().unwrap();
            let colour = cap.get(3).unwrap().as_str();
            contents.push(Content {
                colour: Colour::from(colour),
                count,
            });
        }
    }
    (Colour::from(colour), contents)
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> RuleSet {
    let mut rules = HashMap::new();
    for line in input.lines() {
        let (colour, contents) = parse_rule(line);
        rules.insert(colour, contents);
    }
    RuleSet { rules }
}

#[aoc(day7, part1)]
fn have_gold_bags(ruleset: &RuleSet) -> usize {
    ruleset
        .rules
        .keys()
        .filter(|c| ruleset.contains(c, &Colour::from("shiny gold")))
        .count()
}

#[aoc(day7, part2)]
fn in_gold_bags(ruleset: &RuleSet) -> usize {
    ruleset.count_contents(&Colour::from("shiny gold"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule_with_contents() {
        let actual = parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        let expected = (
            Colour::from("light red"),
            vec![
                Content {
                    colour: Colour::from("bright white"),
                    count: 1,
                },
                Content {
                    colour: Colour::from("muted yellow"),
                    count: 2,
                },
            ],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_rule_without_contents() {
        let actual = parse_rule("faded blue bags contain no other bags.");
        let expected = (Colour::from("faded blue"), Vec::new());
        assert_eq!(actual, expected);
    }
}
