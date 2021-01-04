use counter::Counter;
use itertools::Itertools;
use std::convert::TryFrom;
use std::fmt;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

impl Position {
    fn parse(input: char) -> Result<Self, &'static str> {
        match input {
            '.' => Ok(Position::Floor),
            'L' => Ok(Position::Empty),
            '#' => Ok(Position::Occupied),
            _ => Err("Invalid character."),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct WaitingArea {
    width: usize,
    height: usize,
    layout: Vec<Position>,
}

impl WaitingArea {
    // Starts top-left and then goes cw
    fn adjacent(&self, index: usize, dist: usize) -> impl Iterator<Item = Option<usize>> + '_ {
        let i = isize::try_from(index).unwrap();
        let w = isize::try_from(self.width).unwrap();
        let h = isize::try_from(self.height).unwrap();
        let d = isize::try_from(dist).unwrap();
        let r = i / w;
        let c = i % w;
        let matrix = vec![
            (-d, -d),
            (-d, 0),
            (-d, d),
            (0, d),
            (d, d),
            (d, 0),
            (d, -d),
            (0, -d),
        ];
        matrix.into_iter().map(move |(yi, xi)| {
            let (y, x) = (yi + r, xi + c);
            if x >= 0 && y >= 0 && x < w && y < h {
                Some(usize::try_from(y * w + x).unwrap())
            } else {
                None
            }
        })
    }
    // Finds first chair in direction otherwise empty
    fn first_chair(&self, index: usize, dist: usize) -> impl Iterator<Item = Position> + '_ {
        let mut chairs: Vec<Option<Position>> = self
            .adjacent(index, 1)
            .map(|x| match x {
                Some(i) => Some(self.layout[i]),
                None => None,
            })
            .collect();
        for d in 2..dist + 1 {
            for (dir, i) in self
                .adjacent(index, d)
                .enumerate()
                .filter_map(|(d, i)| match i {
                    Some(index) => Some((d, index)),
                    None => None,
                })
            {
                if let Some(Position::Floor) = chairs[dir] {
                    chairs[dir] = Some(self.layout[i]);
                }
            }
        }
        chairs.into_iter().filter_map(|x| x)
    }
    fn step(&mut self, threshold: usize, dist: usize, dbg: bool) -> bool {
        let current = self.clone();
        for (i, p) in current.layout.iter().enumerate() {
            let count = current
                .first_chair(i, dist)
                .into_iter()
                .collect::<Counter<_>>()
                .into_map();
            match p {
                Position::Empty => {
                    if !count.contains_key(&Position::Occupied) {
                        self.layout[i] = Position::Occupied;
                    }
                }
                Position::Occupied => {
                    if count.contains_key(&Position::Occupied)
                        && count[&Position::Occupied] >= threshold
                    {
                        self.layout[i] = Position::Empty;
                    }
                }
                _ => (),
            }
            if dbg && i >= 50 && i < 60 {
                dbg!(i, &self.layout[i], &count);
            }
        }
        current.layout != self.layout
    }
    fn occupied(&self) -> usize {
        self.layout
            .iter()
            .filter(|&&p| p == Position::Occupied)
            .count()
    }
}

impl fmt::Display for WaitingArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lyt = self.layout.iter().map(|&p| match p {
            Position::Empty => 'L',
            Position::Occupied => '#',
            Position::Floor => '.',
        });
        for line in lyt.chunks(10).into_iter() {
            let s: String = line.collect();
            write!(f, "{}\n", s)?;
        }
        Ok(())
    }
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> WaitingArea {
    let mut width = None;
    let mut height = 0;
    let mut layout = Vec::new();
    for line in input.lines() {
        height += 1;
        if width.is_none() {
            width = Some(line.len());
        }
        assert_eq!(width, Some(line.len()));
        layout.extend(line.chars().map(|c| Position::parse(c).unwrap()));
    }
    WaitingArea {
        width: width.unwrap(),
        height,
        layout,
    }
}

#[aoc(day11, part1)]
fn part1(input: &WaitingArea) -> usize {
    let mut area = input.clone();
    while area.step(4, 1, false) {}
    area.occupied()
}

#[aoc(day11, part2)]
fn part2(input: &WaitingArea) -> usize {
    let mut area = input.clone();
    while area.step(5, 4, false) {}
    area.occupied()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, Position};
    use std::cmp::Ord;

    const INPUT: &str = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL\n";
    const STEP1: &str = "#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##\n";
    const STEP2: &str = "#.LL.L#.##\n#LLLLLL.L#\nL.L.L..L..\n#LLL.LL.L#\n#.LL.LL.LL\n#.LLLL#.##\n..L.L.....\n#LLLLLLLL#\n#.LLLLLL.L\n#.#LLLL.##\n";
    const STEP3: &str = "#.##.L#.##\n#L###LL.L#\nL.#.#..#..\n#L##.##.L#\n#.##.LL.LL\n#.###L#.##\n..#.#.....\n#L######L#\n#.LL###L.L\n#.#L###.##\n";
    const STEP4: &str = "#.#L.L#.##\n#LLL#LL.L#\nL.L.L..#..\n#LLL.##.L#\n#.LL.LL.LL\n#.LL#L#.##\n..L.L.....\n#L#LLLL#L#\n#.LLLLLL.L\n#.#L#L#.##\n";
    const STEP5: &str = "#.#L.L#.##\n#LLL#LL.L#\nL.#.L..#..\n#L##.##.L#\n#.#L.LL.LL\n#.#L#L#.##\n..L.L.....\n#L#L##L#L#\n#.LLLLLL.L\n#.#L#L#.##\n";
    const P2STEP1: &str = "#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##\n";
    const P2STEP2: &str = "#.LL.LL.L#\n#LLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLL#\n#.LLLLLL.L\n#.LLLLL.L#\n";
    const P2STEP3: &str = "#.L#.##.L#\n#L#####.LL\nL.#.#..#..\n##L#.##.##\n#.##.#L.##\n#.#####.#L\n..#.#.....\nLLL####LL#\n#.L#####.L\n#.L####.L#\n";
    const P2STEP4: &str = "#.L#.L#.L#\n#LLLLLL.LL\nL.L.L..#..\n##LL.LL.L#\nL.LL.LL.L#\n#.LLLLL.LL\n..L.L.....\nLLLLLLLLL#\n#.LLLLL#.L\n#.L#LL#.L#\n";
    const P2STEP5: &str = "#.L#.L#.L#\n#LLLLLL.LL\nL.L.L..#..\n##L#.#L.L#\nL.L#.#L.L#\n#.L####.LL\n..#.#.....\nLLL###LLL#\n#.LLLLL#.L\n#.L#LL#.L#\n";
    const P2STEP6: &str = "#.L#.L#.L#\n#LLLLLL.LL\nL.L.L..#..\n##L#.#L.L#\nL.L#.LL.L#\n#.LLLL#.LL\n..#.L.....\nLLL###LLL#\n#.LLLLL#.L\n#.L#LL#.L#\n";

    fn assert_vec_eq<T>(mut left: Vec<T>, mut right: Vec<T>)
    where
        T: Ord,
        T: std::fmt::Debug,
    {
        left.sort_unstable();
        right.sort_unstable();
        assert_eq!(left, right);
    }

    #[test]
    fn test_generator() {
        let area = input_generator(INPUT);
        assert_eq!(area.height, 10);
        assert_eq!(area.width, 10);
        assert_eq!(area.layout[0], Position::Empty);
        assert_eq!(area.layout[1], Position::Floor);
        assert_eq!(area.layout[17], Position::Floor);
        assert_eq!(area.layout[64], Position::Empty);
    }

    #[test]
    fn test_adjacent() {
        let area = input_generator(INPUT);
        assert_vec_eq(
            area.adjacent(0, 1).collect(),
            vec![Some(1), Some(10), Some(11), None, None, None, None, None],
        ); // Top-left
        assert_vec_eq(
            area.adjacent(2, 1).collect(),
            vec![
                Some(1),
                Some(3),
                Some(11),
                Some(12),
                Some(13),
                None,
                None,
                None,
            ],
        ); // Top
        assert_vec_eq(
            area.adjacent(9, 1).collect(),
            vec![Some(8), Some(18), Some(19), None, None, None, None, None],
        ); // Top-right
        assert_vec_eq(
            area.adjacent(40, 1).collect(),
            vec![
                Some(30),
                Some(31),
                Some(41),
                Some(50),
                Some(51),
                None,
                None,
                None,
            ],
        ); // Left
        assert_vec_eq(
            area.adjacent(23, 1).collect(),
            vec![
                Some(12),
                Some(13),
                Some(14),
                Some(22),
                Some(24),
                Some(32),
                Some(33),
                Some(34),
            ],
        ); // Middle
        assert_vec_eq(
            area.adjacent(79, 1).collect(),
            vec![
                Some(68),
                Some(69),
                Some(78),
                Some(88),
                Some(89),
                None,
                None,
                None,
            ],
        ); // Right
        assert_vec_eq(
            area.adjacent(90, 1).collect(),
            vec![Some(80), Some(81), Some(91), None, None, None, None, None],
        ); // Bottom-left
        assert_vec_eq(
            area.adjacent(95, 1).collect(),
            vec![
                Some(84),
                Some(85),
                Some(86),
                Some(94),
                Some(96),
                None,
                None,
                None,
            ],
        ); // Bottom
        assert_vec_eq(
            area.adjacent(99, 1).collect(),
            vec![Some(88), Some(89), Some(98), None, None, None, None, None],
        ); // Bottom-right
        assert_vec_eq(
            area.adjacent(11, 3).collect(),
            vec![Some(14), Some(41), Some(44), None, None, None, None, None],
        );
        assert_vec_eq(
            area.adjacent(55, 4).collect(),
            vec![
                Some(11),
                Some(15),
                Some(19),
                Some(51),
                Some(59),
                Some(91),
                Some(95),
                Some(99),
            ],
        );
        assert_vec_eq(
            area.adjacent(0, 4).collect(),
            vec![Some(4), Some(40), Some(44), None, None, None, None, None],
        );
    }
    #[test]
    fn test_first_chair() {
        let area = input_generator(STEP2);
        assert_vec_eq(
            area.first_chair(0, 1).collect(),
            vec![Position::Floor, Position::Empty, Position::Occupied],
        );
        assert_vec_eq(
            area.first_chair(8, 1).collect(),
            vec![
                Position::Floor,
                Position::Floor,
                Position::Empty,
                Position::Occupied,
                Position::Occupied,
            ],
        );
        println!("{}", STEP2);
        assert_vec_eq(
            area.first_chair(8, 2).collect(),
            vec![
                Position::Floor,
                Position::Empty,
                Position::Occupied,
                Position::Occupied,
                Position::Occupied,
            ],
        );
    }

    #[test]
    fn test_step_p1() {
        let mut area = input_generator(INPUT);
        assert!(area.step(4, 1, false));
        assert_eq!(area, input_generator(STEP1));
        assert!(area.step(4, 1, false));
        assert_eq!(area, input_generator(STEP2));
        assert!(area.step(4, 1, false));
        assert_eq!(area, input_generator(STEP3));
        assert!(area.step(4, 1, false));
        assert_eq!(area, input_generator(STEP4));
        assert!(area.step(4, 1, false));
        assert_eq!(area, input_generator(STEP5));
        assert!(!area.step(4, 1, false));
    }

    #[test]
    fn test_step_p2() {
        let mut area = input_generator(INPUT);
        assert!(area.step(5, 4, false));
        assert_eq!(area, input_generator(P2STEP1));
        assert!(area.step(5, 4, false));
        assert_eq!(area, input_generator(P2STEP2));
        assert!(area.step(5, 4, false));
        assert_eq!(area, input_generator(P2STEP3));
        assert!(area.step(5, 4, false));
        assert_eq!(area, input_generator(P2STEP4));
        assert!(area.step(5, 4, true));
        assert_eq!(area, input_generator(P2STEP5));
        assert!(area.step(5, 4, false));
        assert_eq!(area, input_generator(P2STEP6));
        assert!(!area.step(5, 4, false));
    }
}
