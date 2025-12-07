use std::collections::{HashMap, HashSet};

use aoc2025::get_input;

fn main() {
    let input = get_input(7).unwrap();
    let (part1, part2) = solve(&input);
    println!("{part1}");
    println!("{part2}");
}

fn solve(input: &str) -> (usize, usize) {
    let mut splitters = HashSet::<(usize, usize)>::new();
    let mut start = None;
    let mut depth = 0;

    for (y, line) in input.lines().enumerate() {
        depth += 1;
        for (x, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'.' => {}
                b'^' => {
                    splitters.insert((x, y));
                }
                b'S' => {
                    start = Some((x, y));
                }
                _ => panic!("Unexpected!"),
            }
        }
    }

    let (x, y) = start.unwrap();
    let mut tachyons = HashMap::<usize, usize>::new();
    tachyons.insert(x, 1);

    let mut split_count = 0;

    for d in (y + 1)..=depth {
        let mut new_tachyons = HashMap::<usize, usize>::with_capacity(tachyons.len());

        for (x, count) in tachyons.into_iter() {
            if splitters.contains(&(x, d)) {
                split_count += 1;

                *new_tachyons.entry(x - 1).or_default() += count;
                *new_tachyons.entry(x + 1).or_default() += count;
            } else {
                *new_tachyons.entry(x).or_default() += count;
            }
        }

        tachyons = new_tachyons;
    }

    (split_count, tachyons.values().sum())
}

#[test]
fn test07() {
    let example = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    assert_eq!(solve(example), (21, 40));
}
