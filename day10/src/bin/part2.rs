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
        .map(|(coord, _)| trails(&grid, coord))
        .sum();
    dbg!(ret);
}

fn trails(grid: &Grid, coord: Coord) -> usize {
    let current = grid[coord];
    if current == 9 {
        return 1;
    }
    coord
        .manhattan_adjacent()
        .filter_map(|c| grid.get(c).map(|v| (c, v)))
        .filter(|(_, v)| **v == current + 1)
        .map(|(c, _)| trails(grid, c))
        .sum()
}
