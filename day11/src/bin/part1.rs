fn main() {
    let mut input = aoc::parser::input::<String>()
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut output: Vec<usize> = Vec::new();
    for _ in 0..25 {
        for stone in input.iter() {
            output.append(&mut blink(*stone));
        }
        input = std::mem::take(&mut output);
    }
    println!("{}", input.len());
}

fn blink(n: usize) -> Vec<usize> {
    match n {
        0 => vec![1],
        n if n.ilog10() % 2 == 1 => {
            let half = 10_usize.pow((n.ilog10() + 1) / 2);
            vec![n / half, n % half]
        }
        n => vec![n * 2024],
    }
}
