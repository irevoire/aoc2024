use std::collections::HashSet;

use aoc::{Direction, Turtle};

fn main() {
    let grid = aoc::Grid::from(
        aoc::parser::lines()
            .map(|line: String| line.chars().collect())
            .collect(),
    );
    let guard_pos = grid.position(|pos| *pos == '^').unwrap();
    let mut explored = HashSet::new();
    let mut guard = Turtle::from(guard_pos.sign(), Direction::North);

    loop {
        explored.insert(guard.coord);
        let new_pos = guard.coord + guard.facing;
        if !grid.contains(&new_pos) {
            break;
        }
        if grid[new_pos] == '#' {
            guard.facing = guard.facing.rotate_clockwise();
        } else {
            guard.coord = new_pos;
        }
    }

    println!("{}", explored.len());
}
