use aoc2025::get_input;

fn main() {
    let input = get_input(4).unwrap();
    let (part1, part2) = solve(&input);
    println!("{part1}");
    println!("{part2}");
}

fn solve(input: &str) -> (usize, usize) {
    let mut grid: Vec<Vec<u8>> = input
        .as_bytes()
        .split(|x| *x == b'\n')
        .map(|row| Vec::from(row))
        .collect();
    grid.pop(); // remove last empty line

    let mut initial_state = true;
    let mut initial_can_be_removed = 0;

    let w = grid[0].len();
    let h = grid.len();

    let mut total_removed = 0;
    loop {
        let has_roll =
            |x: usize, y: usize| -> Option<bool> { grid.get(y)?.get(x).map(|l| *l == b'@') };

        let neighbour_roll_count = |x: usize, y: usize| -> usize {
            let mut count = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }

                    let neighbour_has_roll = (|| {
                        let x = x.checked_add_signed(dx)?;
                        let y = y.checked_add_signed(dy)?;
                        has_roll(x, y)
                    })();

                    if let Some(true) = neighbour_has_roll {
                        count += 1;
                    }
                }
            }

            count
        };

        let mut new_grid = Vec::with_capacity(h);
        let mut count_removed = 0;
        for y in 0..h {
            let mut new_row = Vec::with_capacity(w);
            for x in 0..w {
                if has_roll(x, y).unwrap() {
                    if neighbour_roll_count(x, y) < 4 {
                        new_row.push(b'x');
                        count_removed += 1;
                    } else {
                        new_row.push(b'@');
                    }
                } else {
                    new_row.push(b'.');
                }
            }

            new_grid.push(new_row);
        }

        if initial_state {
            initial_can_be_removed = count_removed;
            initial_state = false;
        }

        total_removed += count_removed;

        std::mem::swap(&mut new_grid, &mut grid);

        if count_removed == 0 {
            return (initial_can_be_removed, total_removed);
        }
    }
}

#[test]
fn test04() {
    assert_eq!(
        solve(
            r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
        ),
        (13, 43)
    );
}
