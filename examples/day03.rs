use aoc2025::get_input;

fn main() {
    let input = get_input(3).unwrap();
    let (part1, part2) = solve(&input);
    println!("{part1}");
    println!("{part2}");
}

fn solve(input: &str) -> (usize, usize) {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for line in input.lines() {
        let line = line.as_bytes();

        sum1 += maximize_joltage(line, 2);
        sum2 += maximize_joltage(line, 12);
    }

    (sum1, sum2)
}

fn maximize_joltage(line: &[u8], batteries: usize) -> usize {
    let (i, s) = line
        .iter()
        .enumerate()
        .rev() // rev because last max is returned
        .skip(batteries - 1)
        .max_by_key(|(_, x)| *x)
        .unwrap();

    let e = if batteries > 2 {
        maximize_joltage(&line[i + 1..], batteries - 1)
    } else {
        (line[i + 1..].iter().max().unwrap() - b'0') as usize
    };

    let x = (s - b'0') as usize * 10usize.pow(batteries as u32 - 1) + e;

    x
}

#[test]
fn test03() {
    assert_eq!(
        solve(
            r"987654321111111
811111111111119
234234234234278
818181911112111
"
        ),
        (357, 3121910778619)
    );
}
