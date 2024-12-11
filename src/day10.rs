use std::collections::HashSet;

pub fn problem_1(data: &str) -> i64 {
    let (rows, reachable) = build_reachable(data);
    let mut total = 0;
    let mut set: HashSet<(usize, usize)> = Default::default();
    for r in 0..reachable.len() {
        for c in 0..reachable[0].len() {
            if rows[r][c] != 0 {
                continue;
            }
            set.clear();
            set.extend(&reachable[r][c]);
            total += set.len();
        }
    }
    total.try_into().unwrap()
}

pub fn problem_2(data: &str) -> i64 {
    let (rows, reachable) = build_reachable(data);
    let mut total = 0;
    for r in 0..reachable.len() {
        for c in 0..reachable[0].len() {
            if rows[r][c] != 0 {
                continue;
            }
            total += reachable[r][c].len();
        }
    }
    total.try_into().unwrap()
}

fn build_reachable(data: &str) -> (Vec<Vec<u32>>, Vec<Vec<Vec<(usize, usize)>>>) {
    let rows: Vec<Vec<u32>> = data
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let max_r = rows.len();
    let max_c = rows[0].len();
    let mut reachable: Vec<Vec<Vec<(usize, usize)>>> = rows
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, cell)| if *cell == 9 { vec![(r, c)] } else { vec![] })
                .collect()
        })
        .collect();
    let mut temp: Vec<(usize, usize)> = Default::default();
    for level in (0..9).rev() {
        for r in 0..max_r {
            for c in 0..max_c {
                if rows[r][c] != level {
                    continue;
                }
                let neighbors = Neighbors {
                    max_r,
                    max_c,
                    r,
                    c,
                    next: 0,
                };
                for (next_r, next_c) in neighbors {
                    if rows[next_r][next_c] != level + 1 {
                        continue;
                    }
                    std::mem::swap(&mut temp, &mut reachable[next_r][next_c]);
                    for other in &temp {
                        reachable[r][c].push(*other);
                    }
                    std::mem::swap(&mut temp, &mut reachable[next_r][next_c]);
                }
            }
        }
    }
    (rows, reachable)
}

struct Neighbors {
    max_r: usize,
    max_c: usize,
    c: usize,
    r: usize,
    next: usize,
}

impl Iterator for Neighbors {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let n = self.next;
            self.next += 1;
            match n {
                0 => {
                    if let Some(r) = self.r.checked_sub(1) {
                        return Some((r, self.c));
                    }
                }
                1 => {
                    if let Some(c) = self.c.checked_sub(1) {
                        return Some((self.r, c));
                    }
                }
                2 => {
                    if self.r + 1 < self.max_r {
                        return Some((self.r + 1, self.c));
                    }
                }
                3 => {
                    if self.c + 1 < self.max_c {
                        return Some((self.r, self.c + 1));
                    }
                }
                _ => return None,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA_1: &str = "0123
1234
8765
9876";

    const DATA_2: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_problem_1_data_1() {
        assert_eq!(1, problem_1(DATA_1));
    }

    #[test]
    fn test_problem_1_data_2() {
        assert_eq!(36, problem_1(DATA_2));
    }

    #[test]
    fn test_problem_2_data_1() {
        assert_eq!(16, problem_2(DATA_1));
    }

    #[test]
    fn test_problem_2_data_2() {
        assert_eq!(81, problem_2(DATA_2));
    }
}
