use std::str::FromStr;

use crate::Part;

pub fn run(part: crate::Part) {
    match part {
        Part::A => part_a("src/day01/input.txt"),
        Part::B => part_b("src/day01/input.txt"),
        Part::T => part_b("src/day01/test.txt"),
    }
}

fn part_a(path: &str) {
    let file = std::fs::read_to_string(path).expect("Missing file");
    let mut current_idx = 50;
    let mut times = 0;
    file.lines().for_each(|line| {
        let next_move = line.parse::<NextMove>().expect("Move is broken");

        match next_move.sign {
            Sign::L => {
                current_idx = current_idx - next_move.number;
            }
            Sign::R => {
                current_idx = current_idx + next_move.number;
            }
        }
        current_idx = current_idx.rem_euclid(100);
        if current_idx == 0 {
            times += 1
        }
    });

    dbg!(times);
}

fn part_b(path: &str) {
    let file = std::fs::read_to_string(path).expect("Missing file");
    let mut current_idx = 50;
    let mut times = 0;
    file.lines().for_each(|line| {
        let next_move = line.parse::<NextMove>().expect("Move is broken");
        (times, current_idx) = step(next_move, times, current_idx);
    });

    dbg!(times);
}

fn step(mut next_move: NextMove, mut times: i32, mut current_idx: i32) -> (i32, i32) {
    match next_move.sign {
        Sign::L => {
            while next_move.number > 0 {
                current_idx -= 1;
                next_move.number -= 1;
                if current_idx < 0 {
                    current_idx = 99;
                    times += 1;
                }
            }
        }
        Sign::R => {
            while next_move.number > 0 {
                current_idx += 1;
                next_move.number -= 1;
                if current_idx > 99 {
                    current_idx = 0;
                    times += 1;
                }
            }
        }
    }
    (times, current_idx)
}

struct NextMove {
    sign: Sign,
    number: i32,
}

enum Sign {
    L,
    R,
}

impl FromStr for NextMove {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sign, number_str) = s.split_at(1);
        Ok(Self {
            number: number_str.parse()?,
            sign: match sign {
                "L" => Sign::L,
                _ => Sign::R,
            },
        })
    }
}

#[allow(dead_code)]
impl NextMove {
    fn new(sign: Sign, number: i32) -> Self {
        Self { sign, number }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step() {
        assert_eq!(step(NextMove::new(Sign::L, 100), 0, 50), (1, 50));
        assert_eq!(step(NextMove::new(Sign::L, 500), 0, 50), (5, 50));
        assert_eq!(step(NextMove::new(Sign::R, 500), 0, 50), (5, 50));
        assert_eq!(step(NextMove::new(Sign::R, 50), 0, 50), (1, 0));
        assert_eq!(step(NextMove::new(Sign::L, 50), 0, 50), (1, 0));
    }

    #[test]
    fn test_division() {
        assert_eq!(4i32.div_euclid(6), 0);
        assert_eq!(-4i32.div_euclid(6), 0);
        assert_eq!(-4i32.div_euclid(6), 0);
        assert_eq!((-6i32.div_euclid(6)).abs(), 1);
        assert_eq!((-68i32.div_euclid(6)).abs(), 11);
    }
}
