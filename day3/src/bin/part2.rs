fn try_parse_mul(input: &str) -> Option<(isize, &str)> {
    let input = input.strip_prefix("mul(")?;
    let (left, rem) = input.split_once(',')?;
    let left: isize = left.parse().ok()?;
    let (right, rem) = rem.split_once(')')?;
    let right: isize = right.parse().ok()?;

    Some((left * right, rem))
}

fn try_parse_do(input: &str) -> Option<&str> {
    let input = input.strip_prefix("do()")?;
    Some(input)
}

fn try_parse_dont(input: &str) -> Option<&str> {
    let input = input.strip_prefix("don't()")?;
    Some(input)
}

fn main() {
    let input = aoc::parser::input::<String>();
    let mut input = input.chars();
    let mut ret = 0;
    let mut enabled = true;

    loop {
        if enabled {
            if let Some((mul, rem)) = try_parse_mul(input.as_str()) {
                ret += mul;
                input = rem.chars();
            } else if let Some(rem) = try_parse_dont(input.as_str()) {
                enabled = false;
                let _ = input.next();
                // input = rem.chars();
            } else if input.next().is_none() {
                break;
            }
        } else {
            if let Some(rem) = try_parse_do(input.as_str()) {
                enabled = true;
                let _ = input.next();
                // input = rem.chars();
            }
            if input.next().is_none() {
                break;
            }
        }
    }

    aoc::answer!("{}", ret);
}
