use std::collections::HashSet;

use aoc::{Coord, Grid};

fn main() {
    let grid = Grid::from(
        aoc::parser::lines::<String>()
            .map(|line| line.chars().collect())
            .collect(),
    );

    let mut ret = 0;
    let mut to_explore: HashSet<Coord<isize>> = grid.enumerate().map(|(c, _)| c.sign()).collect();
    while let Some(coord) = to_explore.iter().next().copied() {
        let a = explore(&grid, coord, &mut to_explore);
        ret += a.0 * a.1;
    }

    println!("{ret}");
}

// Return (area, perimeter)
fn explore(
    grid: &Grid<char>,
    coord: Coord<isize>,
    to_explore: &mut HashSet<Coord<isize>>,
) -> (usize, usize) {
    if !to_explore.remove(&coord) {
        return (0, 0);
    }
    let current_value = grid[coord];
    let (mut area, mut perimeter) = (0, 0);
    for next in coord.manhattan_adjacent() {
        if let Some(value) = grid.get(next) {
            if current_value == *value {
                let ret = explore(grid, next, to_explore);
                area += ret.0;
                perimeter += ret.1;
            } else {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }
    }

    (area + 1, perimeter)
}
