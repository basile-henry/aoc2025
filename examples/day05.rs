use aoc2025::{IntervalSet, get_input};

fn main() {
    let input = get_input(5).unwrap();
    let (part1, part2) = solve(&input);
    println!("{part1}");
    println!("{part2}");
}

fn solve(input: &str) -> (usize, usize) {
    let mut it = input.lines();
    let intervals: IntervalSet<usize> = (&mut it)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (s, e) = line.split_once('-').unwrap();
            let start: usize = s.parse().unwrap();
            let end: usize = e.parse().unwrap();

            start..=end
        })
        .collect();

    let mut count = 0;
    for line in it {
        let x = line.parse().unwrap();

        if intervals.contains(&x) {
            count += 1;
        }
    }

    (count, intervals.count())
}

#[test]
fn test05() {
    assert_eq!(
        solve(
            r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
        ),
        (3, 14)
    );
}
