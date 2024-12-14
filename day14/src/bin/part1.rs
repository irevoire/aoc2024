use std::str::FromStr;

use aoc::{Coord, Itertools};

#[derive(Debug)]
struct Robot {
    p: Coord<isize>,
    v: Coord<isize>,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s
            .replace("p=", "")
            .replace("v=", "")
            .split(' ')
            .map(|s| {
                let (x, y) = s
                    .split(',')
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                Coord::at(x, y)
            })
            .collect_tuple()
            .unwrap();
        Ok(Robot { p, v })
    }
}

impl Robot {
    fn pos_in(&self, second: isize) -> Self {
        let p = self.p + self.v * second;
        Self {
            p: Coord::at(p.x.rem_euclid(WIDTH), p.y.rem_euclid(HEIGHT)),
            v: self.v,
        }
    }
}

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

fn main() {
    let setup: Vec<_> = aoc::parser::lines::<Robot>().collect();
    let ret = setup
        .into_iter()
        .map(|robot| robot.pos_in(100))
        .fold([0, 0, 0, 0], |[tl, tr, bl, br], robot| {
            [
                tl + (robot.p.x < WIDTH / 2 && robot.p.y < HEIGHT / 2) as isize,
                tr + (robot.p.x > WIDTH / 2 && robot.p.y < HEIGHT / 2) as isize,
                bl + (robot.p.x < WIDTH / 2 && robot.p.y > HEIGHT / 2) as isize,
                br + (robot.p.x > WIDTH / 2 && robot.p.y > HEIGHT / 2) as isize,
            ]
        })
        .into_iter()
        .reduce(|a, b| a * b)
        .unwrap();

    println!();
    println!("{ret}");
}
