use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    ops::AddAssign,
};

use aoc2025::get_input;

fn main() {
    let input = get_input(11).unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut next_id = 0;
    let mut ids = HashMap::new();
    let mut outputs = HashMap::new();

    let mut get_id = |x| -> usize {
        *ids.entry(x).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        })
    };

    for line in input.lines() {
        let mut it = line.split_whitespace();

        let dev = get_id(it.next().unwrap().strip_suffix(':').unwrap());
        let outs: Vec<usize> = it.map(|x| get_id(x)).collect();
        outputs.insert(dev, outs);
    }

    let you = get_id("you");
    let mut to_visit = VecDeque::new();
    to_visit.push_back(you);

    let mut path_count = HashMap::new();
    path_count.insert(get_id("out"), 1);

    while let Some(node) = to_visit.pop_front() {
        if path_count.contains_key(&node) {
            continue;
        }

        let mut all_solved = true;
        let mut sum = 0;
        for out in outputs.get(&node).unwrap().iter() {
            if let Some(count) = path_count.get(out) {
                sum += count;
            } else {
                all_solved = false;
                to_visit.push_back(*out);
            }
        }

        if all_solved {
            path_count.insert(node, sum);
        } else {
            to_visit.push_back(node);
        }
    }

    path_count[&you]
}

struct Count {
    dac_fft: usize,
    dac: usize,
    fft: usize,
    out_only: usize,
}

impl AddAssign<&Count> for Count {
    fn add_assign(&mut self, rhs: &Count) {
        self.dac_fft += rhs.dac_fft;
        self.dac += rhs.dac;
        self.fft += rhs.fft;
        self.out_only += rhs.out_only;
    }
}

fn part2(input: &str) -> usize {
    let mut next_id = 0;
    let mut ids = HashMap::new();
    let mut outputs = HashMap::new();

    let mut get_id = |x| -> usize {
        *ids.entry(x).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        })
    };

    for line in input.lines() {
        let mut it = line.split_whitespace();

        let dev = get_id(it.next().unwrap().strip_suffix(':').unwrap());
        let outs: Vec<usize> = it.map(|x| get_id(x)).collect();
        outputs.insert(dev, outs);
    }

    let mut to_visit = BTreeSet::new();
    to_visit.insert((0, get_id("svr")));

    let mut path_count = HashMap::new();
    path_count.insert(
        get_id("out"),
        Count {
            dac_fft: 0,
            dac: 0,
            fft: 0,
            out_only: 1,
        },
    );

    while let Some((hops, node)) = to_visit.pop_last() {
        if path_count.contains_key(&node) {
            continue;
        }

        let mut all_solved = true;
        let mut sum = Count {
            dac_fft: 0,
            dac: 0,
            fft: 0,
            out_only: 0,
        };
        for out in outputs.get(&node).unwrap().iter() {
            if let Some(count) = path_count.get(out) {
                sum += count;
                if node == get_id("dac") {
                    sum.dac_fft += count.fft;
                    sum.dac += count.out_only;
                } else if node == get_id("fft") {
                    sum.dac_fft += count.dac;
                    sum.fft += count.out_only;
                }
            } else {
                all_solved = false;
                to_visit.insert((hops + 1, *out));
            }
        }

        if all_solved {
            path_count.insert(node, sum);
        } else {
            to_visit.insert((hops, node));
        }
    }

    path_count[&get_id("svr")].dac_fft
}

#[test]
fn test11() {
    let example1 = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    assert_eq!(part1(example1), 5);

    let example2 = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    assert_eq!(part2(example2), 2);
}
