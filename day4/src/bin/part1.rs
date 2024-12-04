use aoc::{Coord, Grid};

fn main() {
    let grid = Grid::from(
        aoc::parser::lines()
            .map(|line: String| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let ret: usize = grid
        .enumerate()
        .filter(|(_, c)| **c == 'X')
        .map(|(coord, _c)| valid(&grid, coord))
        .sum();
    println!("{ret}");
}

fn valid(grid: &Grid<char>, coord: Coord<usize>) -> usize {
    let mut count = 0;
    let target = "MAS";
    for c in coord.chebyshev_adjacent() {
        let mut coord = Coord::at(coord.x as isize, coord.y as isize);
        let direction = Coord::at(c.x as isize, c.y as isize) - coord;
        for target in target.chars() {
            coord += direction;
            if coord.x < 0
                || coord.y < 0
                || coord.x >= grid.width() as isize
                || coord.y >= grid.height() as isize
            {
                break;
            }
            let coord = Coord::at(coord.x as usize, coord.y as usize);
            if target != grid[coord] {
                break;
            } else if target == grid[coord] && target == 'S' {
                count += 1;
            }
        }
    }

    count
}
