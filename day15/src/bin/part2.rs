use aoc::{Coord, Direction, Grid};

fn main() {
    let input = aoc::parser::input::<String>();
    let (grid, input) = input.split_once("\n\n").unwrap();
    let mut grid = Grid::from(
        grid.replace(".", "..")
            .replace("#", "##")
            .replace("O", "[]")
            .replace("@", "@.")
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect(),
    );
    let input = input.replace("\n", "");

    let mut robot = grid.position(|el| *el == '@').unwrap().sign();

    for direction in input.chars() {
        println!("Move {direction}");
        println!("{grid}");
        println!("\x1b[{}A", grid.height() + 3);
        // std::thread::sleep(std::time::Duration::from_millis(250));

        let direction: Direction = direction.to_string().parse().unwrap();
        if can_move_to(&grid, robot, direction) {
            move_to(&mut grid, &mut robot, direction);
        }
    }

    let score: usize = grid
        .enumerate()
        .filter(|(_, v)| **v == '[')
        .map(|(coord, _)| coord.x + coord.y * 100)
        .sum();

    println!("Finished with a score of: {score}");
    println!("{grid}");
}

fn can_move_to(grid: &Grid<char>, coord: Coord<isize>, direction: Direction) -> bool {
    let target = coord + direction;
    match grid[target] {
        '.' => true,
        '#' => false,
        // In the special case we're moving right or left we should not be blocked by ourselves
        '[' if direction == Direction::Left || direction == Direction::West => {
            can_move_to(grid, target + direction, direction)
        }
        '[' if direction == Direction::Right || direction == Direction::East => {
            can_move_to(grid, target + direction, direction)
        }

        ']' if direction == Direction::Right || direction == Direction::East => {
            can_move_to(grid, target + direction, direction)
        }
        ']' if direction == Direction::Left || direction == Direction::West => {
            can_move_to(grid, target + direction, direction)
        }

        '[' => {
            can_move_to(grid, target, direction)
                && can_move_to(grid, target + Direction::Right, direction)
        }
        ']' => {
            can_move_to(grid, target, direction)
                && can_move_to(grid, target + Direction::Left, direction)
        }
        c => panic!("{c}"),
    }
}

fn move_to(grid: &mut Grid<char>, coord: &mut Coord<isize>, direction: Direction) {
    let target = *coord + direction;
    let robot = grid[*coord] == '@';

    match grid[target] {
        '#' => panic!("Encountered a wall"),
        '[' => {
            move_to(grid, &mut (target + Direction::Right), direction);
            move_to(grid, &mut target.clone(), direction);
        }
        ']' => {
            move_to(grid, &mut (target + Direction::Left), direction);
            move_to(grid, &mut target.clone(), direction);
        }
        '.' => (),
        c => panic!("{c}"),
    }

    grid[target] = grid[*coord];
    grid[*coord] = '.';
    if robot {
        *coord = target;
    }
}
