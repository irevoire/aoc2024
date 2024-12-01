use aoc::num::Distance;

fn main() {
    let (mut left, mut right): (Vec<_>, Vec<_>) = aoc::parser::lines()
        .map(|line: String| {
            let a: Vec<_> = line
                .split(" ")
                .filter_map(|n| n.parse::<isize>().ok())
                .collect();
            (a[0], a[1])
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let answer: isize = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| left.distance(*right))
        .sum();
    aoc::answer!("{}", answer);
}
