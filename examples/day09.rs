use std::collections::BTreeMap;

use aoc2025::get_input;

fn main() {
    let input = get_input(9).unwrap();
    let (part1, part2) = solve(&input);
    println!("{part1}");
    println!("{part2}");
}

fn solve(input: &str) -> (usize, usize) {
    let points: Vec<[usize; 2]> = input
        .lines()
        .map(|line| {
            let p = line
                .split(',')
                .map(|w| w.parse().unwrap())
                .collect::<Vec<usize>>();
            p.try_into().unwrap()
        })
        .collect();

    // compress points to make PIP cheaper
    let x_mapping: BTreeMap<usize, usize> = {
        let mut x_sorted: Vec<usize> = points.iter().map(|[x, _]| *x).collect();
        x_sorted.sort();
        x_sorted.dedup();
        x_sorted
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect()
    };
    let y_mapping: BTreeMap<usize, usize> = {
        let mut y_sorted: Vec<usize> = points.iter().map(|[_, y]| *y).collect();
        y_sorted.sort();
        y_sorted.dedup();
        y_sorted
            .into_iter()
            .enumerate()
            .map(|(i, y)| (y, i))
            .collect()
    };

    // both the example and my input finish with a vertical segment, so I'm ignoring the loopback segment
    let mut horiz_segments: Vec<(usize, (usize, usize))> = points
        .windows(2)
        .filter_map(|w| match w {
            &[[x0, y0], [x1, y1]] => {
                if y0 == y1 {
                    Some((y_mapping[&y0], (x_mapping[&x0], x_mapping[&x1])))
                } else {
                    None
                }
            }
            _ => panic!("Unexpected"),
        })
        .collect();
    horiz_segments.sort_unstable_by_key(|(y, _)| *y);

    let mut max_area = usize::MIN;
    let mut max_restricted_area = usize::MIN;

    for (i, a) in points.iter().enumerate() {
        'rect_test: for b in points.iter().skip(i + 1) {
            // part 1
            let area = (a[0].abs_diff(b[0]) + 1) * (a[1].abs_diff(b[1]) + 1);
            max_area = max_area.max(area);

            // part 2
            if area <= max_restricted_area {
                // skip expensive checks
                continue;
            }

            // compressed bounds
            let x_from = x_mapping[&a[0]].min(x_mapping[&b[0]]);
            let x_to = x_mapping[&a[0]].max(x_mapping[&b[0]]);
            let y_from = y_mapping[&a[1]].min(y_mapping[&b[1]]);
            let y_to = y_mapping[&a[1]].max(y_mapping[&b[1]]);

            for x in x_from..=x_to {
                for y in y_from..=y_to {
                    if !point_inside_polygon(&horiz_segments, [x, y]) {
                        continue 'rect_test;
                    }
                }
            }
            max_restricted_area = max_restricted_area.max(area);
        }
    }

    (max_area, max_restricted_area)
}

fn point_inside_polygon(horiz_segments: &[(usize, (usize, usize))], [x, y]: [usize; 2]) -> bool {
    // Doing PIP by casting a ray downwards from point and looking at intersecting segments
    // select the one with the min y
    if let Some((seg_y, (x0, x1))) = horiz_segments
        .iter()
        .copied()
        .filter(|&(seg_y, (x0, x1))| seg_y >= y && ((x0 <= x && x <= x1) || (x1 <= x && x <= x0)))
        .min_by_key(|(seg_y, _)| *seg_y)
    {
        // example and input seem to be non self intersecting polygon that is described clockwise
        return seg_y == y || (x1 <= x && x <= x0);
    }

    false
}

#[test]
fn test09() {
    let example = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    assert_eq!(solve(example), (50, 24));
}
