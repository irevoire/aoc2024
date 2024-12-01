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

    let mut answer = 0;

    for left in left {
        let count = right
            .iter()
            .take_while(|n| **n <= left)
            .filter(|n| **n == left)
            .count();
        answer += left * count as isize;
    }

    aoc::answer!("{}", answer);
}
