use std::{cmp::Reverse, collections::HashMap};

use aoc2025::get_input;

fn main() {
    let input = get_input(8).unwrap();
    let (part1, part2) = solve(&input, 1000);
    println!("{part1}");
    println!("{part2}");
}

fn dist_sqr([x0, y0, z0]: &[usize; 3], [x1, y1, z1]: &[usize; 3]) -> usize {
    let dx = x0.abs_diff(*x1);
    let dy = y0.abs_diff(*y1);
    let dz = z0.abs_diff(*z1);
    dx * dx + dy * dy + dz * dz
}

fn solve(input: &str, n: usize) -> (usize, usize) {
    let points: Vec<[usize; 3]> = input
        .lines()
        .map(|line| {
            let p = line
                .split(',')
                .map(|w| w.parse().unwrap())
                .collect::<Vec<usize>>();
            p.try_into().unwrap()
        })
        .collect();

    let mut dists = Vec::new();
    for (i, a) in points.iter().enumerate() {
        for b in points.iter().skip(i + 1) {
            dists.push((a, b, dist_sqr(a, b)));
        }
    }
    dists.sort_unstable_by(|(_, _, a), (_, _, b)| a.cmp(b));

    let mut groups = HashMap::<usize, Vec<&[usize; 3]>>::new();
    let mut belongs_to_group = HashMap::<&[usize; 3], usize>::new();
    let mut next_group_id = 0;

    macro_rules! step {
        ($a:ident, $b:ident) => {
            match (
                belongs_to_group.get($a).copied(),
                belongs_to_group.get($b).copied(),
            ) {
                (None, None) => {
                    groups.insert(next_group_id, vec![$a, $b]);
                    belongs_to_group.insert($a, next_group_id);
                    belongs_to_group.insert($b, next_group_id);
                    next_group_id += 1;
                }
                (None, Some(id)) => {
                    groups.get_mut(&id).unwrap().push($a);
                    belongs_to_group.insert($a, id);
                }
                (Some(id), None) => {
                    groups.get_mut(&id).unwrap().push($b);
                    belongs_to_group.insert($b, id);
                }
                (Some(id_a), Some(id_b)) => {
                    if id_a != id_b {
                        // merge groups
                        let mut bs = groups.remove(&id_b).unwrap();
                        for b in bs.iter() {
                            *belongs_to_group.get_mut(b).unwrap() = id_a;
                        }
                        groups.get_mut(&id_a).unwrap().append(&mut bs);
                    }
                }
            }
        };
    }

    let mut it = dists.into_iter();
    for (a, b, _) in (&mut it).take(n) {
        step!(a, b);
    }

    let mut group_sizes: Vec<usize> = groups.values().map(|v| v.len()).collect();
    group_sizes.sort_unstable_by(|a, b| Reverse(a).cmp(&Reverse(b)));

    let out1 = group_sizes[0] * group_sizes[1] * group_sizes[2];
    let mut out2 = 0;

    for (a, b, _) in it {
        step!(a, b);

        if groups.len() == 1 && groups.values().next().unwrap().len() == points.len() {
            out2 = a[0] * b[0];
            break;
        }
    }

    (out1, out2)
}

#[test]
fn test08() {
    let example = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    assert_eq!(solve(example, 10), (40, 25272));
}
