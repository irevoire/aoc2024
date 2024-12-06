use std::collections::HashSet;

use aoc::{Direction, Grid, Turtle};

fn main() {
    let grid = aoc::Grid::from(
        aoc::parser::lines()
            .map(|line: String| line.chars().collect())
            .collect(),
    );
    let guard_pos = grid.position(|pos| *pos == '^').unwrap();
    let guard = Turtle::from(guard_pos.sign(), Direction::North);

    let mut cpt = 0;
    let mut cloned_grid = grid.clone();

    for (coord, _) in grid.enumerate().filter(|(_, c)| **c == '.') {
        cloned_grid[coord] = '#';
        if is_loop(&cloned_grid, guard) {
            cpt += 1;
        }
        cloned_grid[coord] = '.';
    }

    println!("{cpt}");
}

fn is_loop(grid: &Grid<char>, mut guard: Turtle) -> bool {
    let mut explored = HashSet::new();

    loop {
        explored.insert(guard);
        let new_pos = guard.coord + guard.facing;
        if !grid.contains(&new_pos) {
            break;
        }
        if grid[new_pos] == '#' {
            guard.facing = guard.facing.rotate_clockwise();
        } else {
            guard.coord = new_pos;
        }

        if explored.contains(&guard) {
            return true;
        }
    }

    false
}
