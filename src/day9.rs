use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

fn validate(value: usize, preamble: &[usize]) -> Result<usize, usize> {
    for pair in preamble.iter().combinations(2) {
        let sum: usize = pair[..2].iter().map(|&&x| x).sum();
        if sum == value {
            return Ok(value);
        }
    }
    Err(value)
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

#[aoc(day9, part1)]
fn find_num(input: &Vec<usize>) -> Option<usize> {
    for (i, &num) in input[25..].iter().enumerate() {
        if let Err(v) = validate(num, &input[i..i + 25]) {
            return Some(v);
        }
    }
    None
}

#[aoc(day9, part2)]
fn find_contiguous_set(input: &Vec<usize>) -> Option<usize> {
    let invalid = find_num(input)?;
    println!("{}", invalid);
    for (i, &s) in input[0..input.len() - 1].iter().enumerate() {
        let mut smallest = s;
        let mut largest = s;
        let mut sum = s;
        for &e in input[i + 1..input.len()].iter() {
            if e < smallest {
                smallest = e;
            } else if e > largest {
                largest = e;
            }
            sum += e;
            if sum == invalid {
                return Some(smallest + largest);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::validate;

    #[test]
    fn test_validate() {
        let nums = [
            30usize, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
            309, 576,
        ];
        for (i, &num) in nums[5..].iter().enumerate() {
            match validate(num, &nums[i..i + 5]) {
                Ok(_) => (),
                Err(v) if v == 127 => (),
                Err(_) => panic!("Invalid result."),
            }
        }
    }
}
