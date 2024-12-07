struct Report {
    target: isize,
    nums: Vec<isize>,
}

fn main() {
    let ret = aoc::parser::lines::<String>()
        .map(|line| {
            let (target, nums) = line.split_once(':').unwrap();
            Report {
                target: target.parse().unwrap(),
                nums: nums
                    .split(' ')
                    .filter_map(|n| n.parse::<isize>().ok())
                    .collect(),
            }
        })
        .filter(valid)
        .map(|report| report.target)
        .sum::<isize>();

    println!("{ret}");
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

fn valid(report: &Report) -> bool {
    let mut operators = vec![Op::Add; report.nums.len() - 1];

    loop {
        let num = report.nums.iter().skip(1).zip(operators.iter()).fold(
            report.nums[0],
            |acc, (el, op)| match op {
                Op::Add => acc + *el,
                Op::Mul => acc * *el,
            },
        );
        if num == report.target {
            return true;
        }
        if operators.iter().all(|op| *op == Op::Mul) {
            return false;
        }
        for op in operators.iter_mut().rev() {
            if *op == Op::Add {
                *op = Op::Mul;
                break;
            } else {
                *op = Op::Add;
            }
        }
    }
}

// 011100
