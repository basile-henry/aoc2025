use std::collections::{BTreeMap, BTreeSet, HashSet};

use aoc2025::get_input;
use z3::{Optimize, SatResult, ast::Int};

fn main() {
    let input = get_input(10).unwrap();
    let (part1, part2) = solve(&input);
    println!("{part1}");
    println!("{part2}");
}

#[derive(Debug)]
struct Manual {
    light_mask: u16,
    buttons_mask: Vec<u16>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl Manual {
    fn fewest_presses_lights(&self) -> usize {
        let mut seen = HashSet::new();
        let mut queue = BTreeSet::new();

        let initial_lights = 0;
        seen.insert(initial_lights);
        queue.insert((0, initial_lights));

        while let Some((n, lights)) = queue.pop_first() {
            if lights == self.light_mask {
                return n;
            }

            for button in self.buttons_mask.iter() {
                let next_lights = lights ^ *button;
                if !seen.contains(&next_lights) {
                    seen.insert(next_lights);
                    let next = (n + 1, next_lights);
                    queue.insert(next);
                }
            }
        }

        panic!("Unexpected!")
    }

    fn fewest_presses_joltage_z3(&self) -> usize {
        let buttons_ixs: Vec<_> = self
            .buttons
            .iter()
            .map(|btn| HashSet::<usize>::from_iter(btn.iter().map(|x| *x as usize)))
            .collect();
        let buttons_press_count: Vec<_> = (0..self.buttons.len())
            .map(|i| Int::fresh_const(&format!("btn{i}")))
            .collect();

        let solver = Optimize::new();

        for btn in buttons_press_count.iter() {
            solver.assert(&btn.ge(0));
        }

        for (ix, jolt) in self.joltages.iter().enumerate() {
            let buttons: Vec<_> = buttons_press_count
                .iter()
                .zip(buttons_ixs.iter())
                .filter_map(|(btn, ixs)| if ixs.contains(&ix) { Some(btn) } else { None })
                .collect();

            solver.assert(&Int::add(buttons.as_slice()).eq(Int::from_u64(*jolt as u64)));
        }

        let sum = Int::add(buttons_press_count.as_slice());
        solver.minimize(&sum);

        assert!(solver.check(&[]) == SatResult::Sat);
        let model = solver.get_model().unwrap();

        model.eval(&sum, true).unwrap().as_u64().unwrap() as usize
    }

    #[allow(unused)]
    /// too slow, too much RAM :(
    fn fewest_presses_joltage_astar(&self) -> usize {
        let mut seen = BTreeMap::new();
        let mut queue = BTreeSet::new();

        fn heuristic(current: &[usize]) -> usize {
            // current.iter().copied().sum::<usize>()
            current.iter().map(|x| *x * *x).sum::<usize>()
        }

        // solving it in reverse to simplify parts of it
        let initial_joltages = self.joltages.clone().into_boxed_slice();
        seen.insert(initial_joltages.clone(), 0);
        queue.insert((
            0 + heuristic(initial_joltages.as_ref()),
            0,
            initial_joltages,
        ));

        while let Some((_, n, joltages)) = queue.pop_first() {
            if joltages.iter().all(|x| *x == 0) {
                return n;
            }

            for button in self.buttons.iter() {
                if button.iter().any(|x| joltages[*x as usize] == 0) {
                    // can't press this button
                    continue;
                }

                let mut next_joltages = joltages.clone();
                let next_n = n + 1;
                for i in button.iter() {
                    next_joltages[*i as usize] -= 1;
                }

                if next_n < *seen.get(&next_joltages).unwrap_or(&usize::MAX) {
                    if seen.len() % 1_000_000 == 0 {
                        println!(
                            "{} (q{}), {:?}, {:?} => {}",
                            seen.len(),
                            queue.len(),
                            next_joltages.as_ref(),
                            seen.get(&next_joltages),
                            next_n
                        );
                    }

                    seen.insert(next_joltages.clone(), next_n);

                    let h = heuristic(next_joltages.as_ref());
                    let next = (next_n + h, next_n, next_joltages);
                    queue.insert(next);
                }
            }
        }

        panic!("Unexpected!")
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut out1 = 0;
    let mut out2 = 0;

    for (line_ix, line) in input.lines().enumerate() {
        let mut it = line.split_whitespace();
        let light_mask = it
            .next()
            .unwrap()
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap()
            .chars()
            .rev()
            .fold(0, |acc, c| {
                (acc << 1)
                    | match c {
                        '.' => 0b0,
                        '#' => 0b1,
                        _ => panic!("Unexpected char '{c}'"),
                    }
            });
        let mut buttons = Vec::new();
        let mut buttons_mask = Vec::new();
        let mut joltages = Vec::new();

        for section in it {
            if section.starts_with('(') {
                let (button_mask, button) = section
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split(',')
                    .fold((0, Vec::new()), |(acc_mask, mut acc_vec), c| {
                        let x: usize = c.parse().unwrap();
                        acc_vec.push(x);
                        (acc_mask | (1 << x), acc_vec)
                    });
                buttons.push(button);
                buttons_mask.push(button_mask);
            } else {
                for w in section
                    .strip_prefix('{')
                    .unwrap()
                    .strip_suffix('}')
                    .unwrap()
                    .split(',')
                {
                    joltages.push(w.parse().unwrap());
                }
            }
        }

        let manual = Manual {
            light_mask,
            buttons_mask,
            buttons,
            joltages,
        };

        println!("Line {}", line_ix);
        out1 += manual.fewest_presses_lights();
        out2 += dbg!(manual.fewest_presses_joltage_z3());
    }

    (out1, out2)
}

#[test]
fn test10() {
    let example = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    assert_eq!(solve(example), (7, 33));
}
