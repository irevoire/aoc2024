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

    for (i, order) in input.lines().enumerate() {
        println!("{i}/{}", input.len());
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
        if failed {
            use aoc::itertools::Itertools;
            // println!("FAILED {order:?}");
            for permutation in order.iter().cloned().permutations(order.len()) {
                // println!("trying with {permutation:?}");
                let mut forbidden_numbers = HashSet::new();
                let mut failed = false;

                for page in permutation.iter().rev() {
                    if forbidden_numbers.contains(page) {
                        failed = true;
                        break;
                    } else {
                        let forbidden = requires(&rules, *page);
                        forbidden_numbers = forbidden_numbers.union(&forbidden).copied().collect();
                    }
                }

                if !failed {
                    output += permutation[order.len() / 2];
                    break;
                }
            }
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

fn solve(rules: &[(usize, usize)], order: &[usize]) -> Option<Vec<usize>> {
    let mut ret = Vec::new();

    for page in 0..order.len() {}
}
