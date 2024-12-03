fn main() {
    let count = aoc::parser::lines::<String>()
        .filter(|line| {
            let report = line
                .split(' ')
                .filter_map(|s| s.parse::<isize>().ok())
                .collect::<Vec<_>>();
            for i in 0..report.len() {
                let mut report = report.clone();
                report.remove(i);

                let increasing = report[0] < report[1];
                if report.windows(2).all(|r| {
                    (increasing && r[0] < r[1] && r[1] - r[0] <= 3)
                        || (!increasing && r[1] < r[0] && r[0] - r[1] <= 3)
                }) {
                    return true;
                }
            }
            false
        })
        .count();

    aoc::answer!("{}", count);
}
