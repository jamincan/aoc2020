use aoc_runner_derive::{aoc, aoc_generator};

enum Cell {
    Tree,
    Snow,
}

impl From<char> for Cell {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Cell::Tree,
            '.' => Cell::Snow,
            _ => panic!(format!("unexpected character {}", ch)),
        }
    }
}

struct Model {
    height: usize,
    width: usize,
    bitmap: Vec<Cell>,
}

impl std::ops::Index<(usize, usize)> for Model {
    type Output = Cell;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        let Model {
            height,
            width,
            bitmap,
        } = self;
        if row >= *height {
            panic!(format!(
                "Row {} is out of bounds for Model with {} rows.",
                row, height
            ));
        }
        let index = row * *width + (col % *width);
        assert!(index < bitmap.len());
        &bitmap[index]
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Model {
    let mut height: usize = 0;
    let mut width: Option<usize> = None;
    let mut bitmap = Vec::with_capacity(input.len());
    for line in input.lines() {
        height += 1;
        width = match width {
            None => Some(line.len()),
            Some(w) => {
                if w != line.len() {
                    panic!("All input lines for Model must be equal length.")
                }
                Some(w)
            }
        };
        bitmap.extend(line.chars().map(|ch| Cell::from(ch)));
    }
    Model {
        height,
        width: width.unwrap(),
        bitmap,
    }
}

#[aoc(day3, part1)]
fn count_trees_from_corner(input: &Model) -> usize {
    let rows = 0..input.height;
    let cols = (0usize..).step_by(3);
    rows.zip(cols).fold(0usize, |ct, pt| {
        if let Cell::Tree = input[pt] {
            return ct + 1;
        }
        ct
    })
}

#[aoc(day3, part2)]
fn count_trees_with_slopes(input: &Model) -> usize {
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    slopes
        .iter()
        .map(|(r, c)| {
            let rows = (0..input.height).step_by(*r);
            let cols = (0usize..).step_by(*c);
            rows.zip(cols).fold(0usize, |ct, pt| {
                if let Cell::Tree = input[pt] {
                    return ct + 1;
                }
                ct
            })
        })
        .product()
}
