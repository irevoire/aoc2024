use aoc::{Coord, Grid};

fn main() {
    let grid = Grid::from(
        aoc::parser::lines()
            .map(|line: String| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let ret: usize = grid
        .enumerate()
        .filter(|(_, c)| **c == 'A')
        .filter(|(coord, _c)| valid(&grid, *coord))
        .count();
    println!("{ret}");
}

fn valid(grid: &Grid<char>, coord: Coord<usize>) -> bool {
    let top_left = Coord::at(coord.x as isize - 1, coord.y as isize - 1);
    let bottom_right = Coord::at(coord.x as isize + 1, coord.y as isize + 1);
    let top_right = Coord::at(coord.x as isize + 1, coord.y as isize - 1);
    let bottom_left = Coord::at(coord.x as isize - 1, coord.y as isize + 1);
    (top_left.x >= 0
        && top_left.x < grid.width() as isize
        && top_left.y >= 0
        && top_left.y < grid.height() as isize
        && bottom_right.x >= 0
        && bottom_right.x < grid.width() as isize
        && bottom_right.y >= 0
        && bottom_right.y < grid.height() as isize
        && ((grid[Coord::at(top_left.x as usize, top_left.y as usize)] == 'M'
            && grid[Coord::at(bottom_right.x as usize, bottom_right.y as usize)] == 'S')
            || (grid[Coord::at(top_left.x as usize, top_left.y as usize)] == 'S'
                && grid[Coord::at(bottom_right.x as usize, bottom_right.y as usize)] == 'M')))
        && (top_right.x >= 0
            && top_right.x < grid.width() as isize
            && top_right.y >= 0
            && top_right.y < grid.height() as isize
            && bottom_left.x >= 0
            && bottom_left.x < grid.width() as isize
            && bottom_left.y >= 0
            && bottom_left.y < grid.height() as isize
            && ((grid[Coord::at(top_right.x as usize, top_right.y as usize)] == 'M'
                && grid[Coord::at(bottom_left.x as usize, bottom_left.y as usize)] == 'S')
                || (grid[Coord::at(top_right.x as usize, top_right.y as usize)] == 'S'
                    && grid[Coord::at(bottom_left.x as usize, bottom_left.y as usize)] == 'M')))
}
