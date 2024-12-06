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
        let mut order: Vec<usize> = order
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        if errors_at(&rules, &order).is_some() {
            while let Some(idx) = errors_at(&rules, &order) {
                order.swap(idx, idx + 1);
            }

            output += order[order.len() / 2];
        }
    }

    println!("{output}");
}

fn errors_at(rules: &[(usize, usize)], order: &[usize]) -> Option<usize> {
    let mut forbidden_numbers = HashSet::new();

    for (idx, page) in order.iter().enumerate().rev() {
        if forbidden_numbers.contains(page) {
            return Some(idx);
        } else {
            let forbidden = requires(&rules, *page);
            forbidden_numbers = forbidden_numbers.union(&forbidden).copied().collect();
        }
    }
    None
}

fn requires(rules: &[(usize, usize)], page: usize) -> HashSet<usize> {
    rules
        .iter()
        .filter(|(n, _)| *n == page)
        .map(|(_, p)| *p)
        .collect()
}
