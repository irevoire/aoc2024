use std::collections::HashSet;

use aoc::{parser, Coord, Grid};

fn main() {
    let grid = Grid::from(
        parser::lines()
            .map(|line: String| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>()
            })
            .collect(),
    );
    let ret: usize = grid
        .enumerate()
        .filter(|(_, v)| **v == 0)
        .map(|(coord, _)| trails(&grid, coord, &mut HashSet::new()))
        .sum();
    dbg!(ret);
}

fn trails(grid: &Grid, coord: Coord, already_done: &mut HashSet<Coord>) -> usize {
    let current = grid[coord];
    if current == 9 {
        if already_done.insert(coord) {
            return 1;
        } else {
            return 0;
        }
    }
    coord
        .manhattan_adjacent()
        .filter_map(|c| grid.get(c).map(|v| (c, v)))
        .filter(|(_, v)| **v == current + 1)
        .map(|(c, _)| trails(grid, c, already_done))
        .sum()
}
