fn main() {
    let input = aoc::parser::input::<String>();
    let mut expand = Vec::new();
    let mut free = false;
    let mut idx = 0;

    for c in input.trim().chars() {
        let n: usize = c.to_string().parse().unwrap();
        if free {
            expand.push((n, ".".to_string()));
        } else {
            expand.push((n, idx.to_string()));
            idx += 1;
        }
        free = !free;
    }

    let mut idx = expand.len() - 1;

    loop {
        let (size, current) = expand[idx].clone();
        if current != "." {
            if let Some((i, (sub_size, _end))) = expand
                .iter()
                .enumerate()
                .take_while(|(i, _)| *i < idx)
                .find(|(_, (s, c))| *c == "." && size <= *s)
            {
                let sub_size = *sub_size;
                expand[i] = expand[idx].clone();
                expand[idx].1 = ".".to_string();

                if size < sub_size {
                    expand.insert(i + 1, (sub_size - size, ".".to_string()));
                }
            } else if idx == 0 {
                break;
            }
        }
        idx -= 1;
    }

    let mut ret = Vec::new();
    for (size, current) in expand.iter() {
        for _ in 0..*size {
            ret.push(current);
        }
    }

    let checksum: usize = ret
        .iter()
        // .filter(|id| id.as_str() != ".")
        .enumerate()
        .filter(|(_, id)| id.as_str() != ".")
        .map(|(i, id)| i * id.to_string().parse::<usize>().unwrap())
        .sum();
    dbg!(checksum);
}

// 1 2 3 4 5
// 022111222
