use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
struct BoardingPass {
    row: usize,
    col: usize,
    id: usize,
}

impl From<&str> for BoardingPass {
    fn from(input: &str) -> Self {
        assert_eq!(input.len(), 10);
        let row = input[..7].chars().fold(0, |mut row, ch| {
            assert!(ch == 'F' || ch == 'B');
            row <<= 1;
            if ch == 'B' {
                row += 1;
            }
            row
        });
        let col = input[7..].chars().fold(0, |mut col, ch| {
            assert!(ch == 'L' || ch == 'R');
            col <<= 1;
            if ch == 'R' {
                col += 1;
            }
            col
        });
        let id = row * 8 + col;
        BoardingPass { row, col, id }
    }
}

impl From<(usize, usize)> for BoardingPass {
    fn from((row, col): (usize, usize)) -> Self {
        BoardingPass {
            row,
            col,
            id: row * 8 + col,
        }
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<BoardingPass> {
    input.lines().map(|line| BoardingPass::from(line)).collect()
}

#[aoc(day5, part1)]
fn highest_seat(passes: &Vec<BoardingPass>) -> Option<usize> {
    passes.iter().map(|pass| pass.id).max()
}

#[aoc(day5, part2)]
fn missing_seat(passes: &Vec<BoardingPass>) -> Option<usize> {
    let mut ids: Vec<usize> = passes.iter().map(|pass| pass.id).collect();
    ids.sort_unstable();
    // cur: ids[1..], prev: ids
    for (&cur, &prev) in ids[1..].iter().zip(ids.iter()) {
        if cur != prev + 1 {
            return Some(cur - 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_boardpasses() {
        assert_eq!(
            BoardingPass::from("FBFBBFFRLR"),
            BoardingPass::from((44, 5))
        );
        assert_eq!(
            BoardingPass::from("BFFFBBFRRR"),
            BoardingPass::from((70, 7))
        );
        assert_eq!(
            BoardingPass::from("FFFBBBFRRR"),
            BoardingPass::from((14, 7))
        );
        assert_eq!(
            BoardingPass::from("BBFFBBFRLL"),
            BoardingPass::from((102, 4))
        );
    }
}
