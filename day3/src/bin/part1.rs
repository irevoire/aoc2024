fn try_parse(input: &str) -> Option<(isize, &str)> {
    let input = input.strip_prefix("mul(")?;
    let (left, rem) = input.split_once(',')?;
    let left: isize = left.parse().ok()?;
    let (right, rem) = rem.split_once(')')?;
    let right: isize = right.parse().ok()?;

    Some((left * right, rem))
}

fn main() {
    let input = aoc::parser::input::<String>();
    let mut input = input.chars();
    let mut ret = 0;

    loop {
        if let Some((mul, rem)) = try_parse(input.as_str()) {
            ret += mul;
            input = rem.chars();
        } else if input.next().is_none() {
            break;
        }
    }

    aoc::answer!("{}", ret);
}
