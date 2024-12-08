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
            antinodes.insert(coord.sign());
            antinodes.insert(coord2.sign());
            let distance = coord2.sign() - coord.sign();
            let mut left = coord.sign() - distance;
            while grid.contains(&left) {
                antinodes.insert(left);
                left -= distance;
            }
            let mut right = coord2.sign() + distance;
            while grid.contains(&right) {
                antinodes.insert(right);
                right -= distance;
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
