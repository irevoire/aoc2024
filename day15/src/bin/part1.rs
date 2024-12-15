use aoc::{Coord, Direction, Grid};

fn main() {
    let input = aoc::parser::input::<String>();
    let (grid, input) = input.split_once("\n\n").unwrap();
    let mut grid = Grid::from(
        grid.trim()
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect(),
    );
    let input = input.replace("\n", "");

    let mut robot = grid.position(|el| *el == '@').unwrap().sign();
    grid[robot] = '.';

    for direction in input.chars() {
        grid[robot] = '@';
        println!("Move {direction}");
        println!("{grid}");
        println!("\x1b[{}A", grid.height() + 3);
        // std::thread::sleep(std::time::Duration::from_millis(250));
        grid[robot] = '.';

        let direction: Direction = direction.to_string().parse().unwrap();
        move_to(&mut grid, &mut robot, direction);
    }

    let score: usize = grid
        .enumerate()
        .filter(|(_, v)| **v == 'O')
        .map(|(coord, _)| coord.x + coord.y * 100)
        .sum();

    println!("Finished with a score of: {score}");
    println!("{grid}");
}

fn move_to(grid: &mut Grid<char>, robot: &mut Coord<isize>, direction: Direction) {
    let next = std::iter::successors(Some(*robot + direction), |coord| Some(*coord + direction))
        .find(|coord| grid[coord] != 'O')
        .unwrap();
    if grid[next] == '#' {
        return;
    }
    *robot += direction;
    grid[next] = 'O';
    grid[*robot] = '.';
}
