use std::collections::HashMap;

// (the stone value, after n steps) => becomes that many stones
type Cache = HashMap<(usize, usize), usize>;

fn main() {
    let input = aoc::parser::input::<String>()
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut cache: Cache = HashMap::default();
    let mut count: usize = 0;
    for stone in input {
        count += blink(stone, 74, &mut cache);
    }
    println!("{count}");
}

fn blink(stone: usize, n_time: usize, cache: &mut Cache) -> usize {
    if let Some(ret) = cache.get(&(stone, n_time)) {
        return *ret;
    }
    let ret = match stone {
        0 => vec![1],
        n if n.ilog10() % 2 == 1 => {
            let half = 10_usize.pow((n.ilog10() + 1) / 2);
            vec![n / half, n % half]
        }
        n => vec![n * 2024],
    };
    let ret = if n_time == 0 {
        ret.len()
    } else {
        let mut count = 0;
        for stone in ret {
            count += blink(stone, n_time - 1, cache);
        }
        count
    };
    cache.insert((stone, n_time), ret);
    ret
}
