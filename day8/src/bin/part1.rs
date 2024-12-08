use std::collections::HashSet;

use aoc::Grid;

fn main() {
    let grid = Grid::from(
        aoc::parser::lines()
            .map(|line: String| line.chars().collect::<Vec<_>>())
            .collect(),
    );
    let mut antinodes = HashSet::new();

    for (coord, value) in grid.enumerate().filter(|(_, v)| **v != '.') {
        for coord2 in grid
            .enumerate()
            .filter(|(_, v)| *v == value)
            .filter(|(c, _)| *c != coord)
            .map(|(c, _)| c)
        {
            let distance = coord2.sign() - coord.sign();
            let left = coord.sign() - distance;
            if grid.contains(&left) {
                antinodes.insert(left);
            }
            let right = coord2.sign() + distance;
            if grid.contains(&right) {
                antinodes.insert(right);
            }
        }
    }

    let grid = grid.map_with_coord(|coord, el| {
        if antinodes.contains(&coord.sign()) {
            '#'
        } else {
            el
        }
    });
    println!("{}", grid);
    println!("{}", antinodes.len());
}
