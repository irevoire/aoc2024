use std::collections::HashSet;

fn main() {
    let input = aoc::parser::input::<String>();
    let (rules, input) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('|').unwrap();
            (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut output = 0;

    for order in input.lines() {
        let order: Vec<usize> = order
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let mut forbidden_numbers = HashSet::new();

        let mut failed = false;
        for page in order.iter().rev() {
            if forbidden_numbers.contains(page) {
                failed = true;
                break;
            } else {
                let forbidden = requires(&rules, *page);
                forbidden_numbers = forbidden_numbers.union(&forbidden).copied().collect();
            }
        }
        if !failed {
            output += order[order.len() / 2];
        }
    }

    println!("{output}");
}

fn requires(rules: &[(usize, usize)], page: usize) -> HashSet<usize> {
    rules
        .iter()
        .filter(|(n, _)| *n == page)
        .map(|(_, p)| *p)
        .collect()
}
