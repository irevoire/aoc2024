use std::str::FromStr;

use aoc::{Coord, Itertools};

#[derive(Debug)]
struct ClawMachine {
    a: Coord<i128>,
    b: Coord<i128>,
    target: Coord<i128>,
}

impl FromStr for ClawMachine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b, prize) = s
            .lines()
            .map(|button| {
                let (_, coords) = button.split_once(':').unwrap();
                let (x, y) = coords.trim().split_once(", ").unwrap();
                let x = x
                    .strip_prefix("X+")
                    .or(x.strip_prefix("X="))
                    .unwrap()
                    .parse::<i128>()
                    .unwrap();
                let y = y
                    .strip_prefix("Y+")
                    .or(y.strip_prefix("Y="))
                    .unwrap()
                    .parse::<i128>()
                    .unwrap();

                Coord::at(x, y)
            })
            .collect_tuple()
            .unwrap();

        Ok(ClawMachine {
            a,
            b,
            target: prize + 10000000000000,
        })
    }
}

impl ClawMachine {
    pub fn solve(&self) -> Option<i128> {
        let b = ((self.target.y * self.a.x) - (self.a.y * self.target.x))
            / ((self.b.y * self.a.x) - (self.a.y * self.b.x));
        let a = (self.target.x - self.b.x * b) / self.a.x;

        // dbg!((a, b));

        if self.target == self.a * a + self.b * b {
            Some(a * 3 + b)
        } else {
            None
        }
    }
}

fn main() {
    let input = aoc::parser::input::<String>();
    let ret: i128 = input
        .split("\n\n")
        .map(|machine| machine.parse::<ClawMachine>().unwrap())
        // .skip(20)
        // .take(1)
        .filter_map(|machine| machine.solve())
        .sum();
    println!("{ret}");
}
