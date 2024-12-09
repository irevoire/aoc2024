fn main() {
    let input = aoc::parser::input::<String>();
    let mut expand = Vec::new();
    let mut free = false;
    let mut idx = 0;

    for c in input.trim().chars() {
        let n: usize = c.to_string().parse().unwrap();
        if free {
            for _ in 0..n {
                expand.push(".".to_string());
            }
        } else {
            for _ in 0..n {
                expand.push(idx.to_string());
            }
            idx += 1;
        }
        free = !free;
    }

    let mut ret = Vec::new();
    let mut iter = expand.iter();

    while let Some(id) = iter.next() {
        if *id == "." {
            if let Some(end) = iter.by_ref().rev().find(|c| **c != ".") {
                ret.push(end);
            } else {
                println!("broke here");
                break;
            }
        } else {
            ret.push(id);
        }
    }

    let checksum: usize = ret
        .iter()
        .enumerate()
        .map(|(i, id)| i * id.to_string().parse::<usize>().unwrap())
        .sum();
    dbg!(checksum);
}

// 1 2 3 4 5
// 022111222
