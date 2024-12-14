use std::{io::Read, str::FromStr};

use aoc::termion::{event::Key, input::TermRead};
use std::io::{stdin, stdout, Write};
use termion::raw::IntoRawMode;

use aoc::{Coord, Grid, Itertools};

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
    let mut seconds = 161;

    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = stdin();

    let setup: Vec<_> = aoc::parser::lines::<Robot>().collect();
    let mut grid = Grid::from(vec![vec!['.'; WIDTH as usize]; HEIGHT as usize]);

    loop {
        std::thread::sleep_ms(50);
        let ret = setup.iter().map(|robot| robot.pos_in(seconds));
        for robot in ret.clone() {
            grid[robot.p] = '#';
        }
        let s = format!("{grid}");
        let s = s.replace("\n", "\n\r");
        write!(stdout, "{s}\n\r").unwrap();
        write!(stdout, "After {seconds} seconds\n\r").unwrap();
        stdout.flush().unwrap();
        write!(stdout, "\x1b[{}A", HEIGHT + 2).unwrap();
        for robot in ret {
            grid[robot.p] = '.';
        }

        for key in stdin.by_ref().keys() {
            match key.unwrap() {
                Key::Char('a') => seconds -= 103,
                Key::Char('e') => seconds += 103,
                Key::Char('c') => return,
                _ => (),
            }
            break;
        }
    }
}
