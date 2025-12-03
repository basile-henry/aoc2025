use aoc2025::get_input;

fn main() {
    let input = get_input(2).unwrap();
    let (part1, part2) = solve(&input);
    println!("{part1}");
    println!("{part2}");
}

fn solve(input: &str) -> (usize, usize) {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for range in input.trim().split(',') {
        let (start, end) = range.split_once('-').unwrap();

        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();

        for x in start..=end {
            if let Some(r) = lowest_repeat_count(&format!("{}", x)) {
                if r == 2 {
                    sum1 += x;
                }

                sum2 += x;
            }
        }
    }

    (sum1, sum2)
}

fn lowest_repeat_count(x: &str) -> Option<usize> {
    let l = x.len();

    'chunk_loop: for chunk_size in (1..=l / 2).rev() {
        if l % chunk_size != 0 {
            continue;
        }

        for chunk in x.as_bytes().chunks_exact(chunk_size) {
            if chunk != &x.as_bytes()[..chunk_size] {
                continue 'chunk_loop;
            }
        }

        return Some(l / chunk_size);
    }

    None
}

#[test]
fn test02() {
    assert_eq!(
        solve(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124\n"
        ),
        (1227775554, 4174379265)
    );
}
