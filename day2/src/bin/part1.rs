fn main() {
    let count = aoc::parser::lines::<String>()
        .filter(|line| {
            let report = line
                .split(' ')
                .filter_map(|s| s.parse::<isize>().ok())
                .collect::<Vec<_>>();
            let increasing = report[0] < report[1];
            report.windows(2).all(|r| {
                (increasing && r[0] < r[1] && r[1] - r[0] <= 3)
                    || (!increasing && r[1] < r[0] && r[0] - r[1] <= 3)
            })
        })
        .count();

    aoc::answer!("{}", count);
}
