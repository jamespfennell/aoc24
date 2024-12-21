use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

use crate::common::{Direction, Point};

const KEYPAD_D: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];
const KEYPAD_N: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

pub fn problem_1(data: &str) -> usize {
    solve(data, 2)
}

pub fn problem_2(data: &str) -> usize {
    solve(data, 25)
}

pub fn solve(data: &str, keypad_ds_levels: usize) -> usize {
    let keypad_d_paths = build_paths(KEYPAD_D);
    let keypad_n_paths = build_paths(KEYPAD_N);
    data.lines()
        .map(|line| {
            let l =
                shortest_path_for_n_code(line, &keypad_n_paths, &keypad_d_paths, keypad_ds_levels);
            let n: usize = line[0..line.len() - 1].parse().unwrap();
            l * n
        })
        .sum()
}

type Paths = HashMap<(char, char), Vec<Vec<Direction>>>;
type Cache = HashMap<(char, char, usize), usize>;

fn shortest_path_for_n_code(
    code: &str,
    keypad_n_paths: &Paths,
    keypad_d_paths: &Paths,
    keypad_ds_levels: usize,
) -> usize {
    let mut cache: Cache = Default::default();
    // Starting from A, we need to type chars on the first key pad.
    // The final A is already contained in chars.
    let chars: Vec<char> = code.chars().collect();
    let mut sum = 0_usize;
    let mut current = 'A';
    for i in 0..chars.len() {
        let next = chars[i];
        let codes = keypad_n_paths.get(&(current, next)).unwrap();
        sum += codes
            .into_iter()
            .map(|code| {
                let l =
                    shortest_path_for_d_code(code, keypad_d_paths, &mut cache, keypad_ds_levels);
                l
            })
            .min()
            .unwrap();
        current = next;
    }
    sum
}

fn shortest_path_for_d_code(
    code: &[Direction],
    keypad_d_paths: &Paths,
    cache: &mut Cache,
    keypad_ds_level: usize,
) -> usize {
    // Starting from A, we need to type code and then 'A' on the second key pad
    let mut current = 'A';
    let mut sum = 0;
    for i in 0..=code.len() {
        let next = code.get(i).map(Direction::char).unwrap_or('A');
        sum += match cache.get(&(current, next, keypad_ds_level)) {
            Some(n) => *n,
            None => {
                let codes = keypad_d_paths.get(&(current, next)).unwrap();
                let n = codes
                    .into_iter()
                    .map(|code| {
                        let l = if keypad_ds_level == 1 {
                            code.len() + 1
                        } else {
                            shortest_path_for_d_code(
                                code,
                                keypad_d_paths,
                                cache,
                                keypad_ds_level - 1,
                            )
                        };
                        l
                    })
                    .min()
                    .unwrap();
                cache.insert((current, next, keypad_ds_level), n);
                n
            }
        };
        current = next;
    }
    sum
}

fn build_paths<const R: usize, const C: usize>(
    keypad: [[char; C]; R],
) -> HashMap<(char, char), Vec<Vec<Direction>>> {
    let mut edges: HashMap<Point, Vec<Direction>> = Default::default();
    for r in 0..R {
        for c in 0..C {
            if keypad[r][c] == ' ' {
                continue;
            }
            let p = Point(r, c);
            let edges = edges.entry(p).or_default();
            for direction in Direction::all_directions() {
                let Some((r, c)) = direction.next_checked((p.0, p.1), (R, C)) else {
                    continue;
                };
                if keypad[r][c] == ' ' {
                    continue;
                }
                edges.push(direction);
            }
        }
    }

    let mut choices: HashMap<(Point, Point), Vec<Vec<Direction>>> = Default::default();
    for start in edges.keys().copied() {
        let mut pending: VecDeque<Point> = Default::default();
        let mut seen: HashSet<Point> = Default::default();
        pending.push_back(start);
        seen.insert(start);
        choices.insert((start, start), vec![vec![]]);
        while let Some(this) = pending.pop_front() {
            let mut paths = vec![];
            for direction in edges.get(&this).unwrap() {
                let other = direction.next((this.0, this.1));
                let other = Point(other.0, other.1);
                if !seen.contains(&other) {
                    pending.push_back(other);
                    seen.insert(other);
                }
                if let Some(in_paths) = choices.get(&(start, other)) {
                    for in_path in in_paths {
                        let mut this_path = in_path.clone();
                        this_path.push(direction.opposite());
                        paths.push(this_path);
                    }
                }
            }
            if this != start {
                choices.insert((start, this), paths);
            }
        }
    }

    choices
        .into_iter()
        .map(|(k, v)| ((keypad[k.0 .0][k.0 .1], keypad[k.1 .0][k.1 .1]), v))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "029A
980A
179A
456A
379A";

    super::super::tests::tests!((test_problem_1_data_1, problem_1, DATA, 126384),);
}
