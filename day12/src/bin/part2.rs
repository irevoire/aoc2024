use std::collections::HashSet;

use aoc::{Coord, Direction, Grid};

fn main() {
    let grid = Grid::from(
        aoc::parser::lines::<String>()
            .map(|line| line.chars().collect())
            .collect(),
    );

    let mut ret = 0;
    let mut to_explore: HashSet<Coord<isize>> = grid.enumerate().map(|(c, _)| c.sign()).collect();
    while let Some(coord) = to_explore.iter().next().copied() {
        let (area, edges) = explore(&grid, coord, &mut to_explore);
        let sides = count_edges(edges);
        ret += area * sides;
    }

    println!("{ret}");
}

#[derive(Debug, PartialEq, Eq)]
struct Edge {
    coord: Coord<isize>,
    direction: Direction,
}

fn count_edges(mut edges: Vec<Edge>) -> usize {
    let mut count = 0;

    while let Some(edge) = edges.pop() {
        remove_edge(edge, &mut edges);
        count += 1;
    }

    count
}

fn remove_edge(base: Edge, edges: &mut Vec<Edge>) {
    let left = Edge {
        coord: base.coord + base.direction.rotate_clockwise(),
        direction: base.direction,
    };
    let right = Edge {
        coord: base.coord + base.direction.rotate_anti_clockwise(),
        direction: base.direction,
    };

    if let Some(idx) = edges.iter().position(|edge| *edge == left) {
        edges.remove(idx);
        remove_edge(left, edges);
    }
    if let Some(idx) = edges.iter().position(|edge| *edge == right) {
        edges.remove(idx);
        remove_edge(right, edges);
    }
}

// Return (area, perimeter)
fn explore(
    grid: &Grid<char>,
    coord: Coord<isize>,
    to_explore: &mut HashSet<Coord<isize>>,
) -> (usize, Vec<Edge>) {
    if !to_explore.remove(&coord) {
        return (0, Vec::new());
    }

    let current_value = grid[coord];
    let (mut area, mut edges) = (0, Vec::new());
    for (next, direction) in coord.manhattan_adjacent_with_direction() {
        if let Some(value) = grid.get(next) {
            if current_value == *value {
                let mut ret = explore(grid, next, to_explore);
                area += ret.0;
                edges.append(&mut ret.1);
            } else {
                edges.push(Edge { coord, direction });
            }
        } else {
            edges.push(Edge { coord, direction });
        }
    }

    (area + 1, edges)
}
