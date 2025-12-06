use aoc2025::get_input;

fn main() {
    let input = get_input(6).unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut accum: Vec<(usize, usize)> = Vec::new();
    let mut out = 0;

    for line in input.lines() {
        for (i, elem) in line.trim().split_whitespace().enumerate() {
            out += match elem {
                "*" => accum[i].1,
                "+" => accum[i].0,
                _ => {
                    let x = elem.parse().unwrap();

                    if let Some((s, p)) = accum.get_mut(i) {
                        *s += x;
                        *p *= x;
                    } else {
                        accum.push((x, x));
                    }

                    0
                }
            };
        }
    }

    out
}

fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let mut accum: Vec<Option<usize>> = Vec::new();

    for line in input.split(|b| *b == b'\n') {
        if matches!(line[0], b'*' | b'+') {
            let mut out = 0;

            let mut cur = None;
            let mut intermediate = 0;

            for (e, a) in line.iter().zip(accum.into_iter()) {
                match *e {
                    b'*' => {
                        cur = Some(b'*');
                        out += intermediate;
                        intermediate = 1;
                    }
                    b'+' => {
                        cur = Some(b'+');
                        out += intermediate;
                        intermediate = 0;
                    }
                    _ => {}
                }

                if let Some(a) = a {
                    if let Some(b'*') = cur {
                        intermediate *= a;
                    } else if let Some(b'+') = cur {
                        intermediate += a;
                    } else {
                        unreachable!()
                    }
                }
            }

            out += intermediate;

            return out;
        }

        for (i, elem) in line.iter().enumerate() {
            if *elem == b' ' {
                accum.push(None);
                continue;
            }

            let digit = (elem - b'0') as usize;

            if let Some(Some(x)) = accum.get_mut(i) {
                *x *= 10;
                *x += digit;
            } else if let Some(x) = accum.get_mut(i) {
                *x = Some(digit);
            } else {
                accum.push(Some(digit));
            }
        }
    }

    unreachable!()
}

#[test]
fn test06() {
    let example = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    assert_eq!(part1(example), 4277556);
    assert_eq!(part2(example), 3263827);
}
