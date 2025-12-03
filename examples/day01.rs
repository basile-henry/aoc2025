use std::ops::Neg;

use aoc2025::get_input;

fn main() {
    let input = get_input(1).unwrap();
    let (part1, part2) = solve(&input);
    println!("{part1}");
    println!("{part2}");
}

fn solve(input: &str) -> (usize, usize) {
    let mut pos: i16 = 50;
    let mut stop_zero = 0;
    let mut click_zero = 0;

    for line in input.lines() {
        if let Some(rest) = line.strip_prefix("L") {
            let n = rest.parse::<i16>().unwrap();
            let prev_pos = pos;
            pos -= n;

            if pos < 0 {
                click_zero += pos.neg().div_euclid(100) as usize;
                if prev_pos > 0 {
                    click_zero += 1;
                }
            } else if pos == 0 {
                click_zero += 1;
            }
        } else if let Some(rest) = line.strip_prefix("R") {
            let n = rest.parse::<i16>().unwrap();
            pos += n;
            click_zero += pos.div_euclid(100) as usize;
        } else {
            unreachable!()
        };

        pos = pos.rem_euclid(100);

        if pos == 0 {
            stop_zero += 1;
        }
    }

    (stop_zero, click_zero)
}

#[test]
fn test01() {
    assert_eq!(
        solve(
            r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
        ),
        (3, 6)
    );
    assert_eq!(solve("R1000\n"), (0, 10));
}
