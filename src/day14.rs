use std::collections::HashSet;

use regex::Regex;

pub fn problem_1(data: &str) -> i64 {
    let robots = parse_robots(data);
    let max_x = 101;
    let max_y = 103;
    simulate_robots(&robots, max_x, max_y)
}

pub fn problem_2(data: &str) -> i64 {
    let mut robots = parse_robots(data);
    let max_x = 101;
    let max_y = 103;
    let mut positions: HashSet<(i64, i64)> = Default::default();
    let mut line = String::new();
    for t in 0..2 {
        positions.clear();
        for r in &robots {
            positions.insert((r.p.0, r.p.1));
        }
        // From running experiments:
        // 620 - 519 - 418  # interesting vertical structure every 14 + 101 * m
        // 579 - 476 - 373  # interesting horizontal structure every 64 + 103 * n
        // We want a T such that T = 14 + 101 * m = 64 + 103 * n for integers n and m
        // The smallest such n is 76, and t = 64 + 103 * 76 = 7892
        println!("------------- {t}");
        for y in 0..max_y {
            line.clear();
            for x in 0..max_x {
                let c = if positions.contains(&(x, y)) {
                    'X'
                } else {
                    ' '
                };
                line.push(c);
            }
            println!("{line}");
        }

        for r in &mut robots {
            r.travel(max_x, max_y, 64 + 103 * 76);
        }
    }
    7892
}

fn simulate_robots(robots: &[Robot], max_x: i64, max_y: i64) -> i64 {
    let mid_x = (max_x - 1) / 2;
    let mid_y = (max_y - 1) / 2;

    let num_steps = 100;
    let mut sums: [i64; 4] = [0, 0, 0, 0];
    for r in robots {
        let end_x = (r.p.0 + r.v.0 * num_steps).rem_euclid(max_x);
        let end_y = (r.p.1 + r.v.1 * num_steps).rem_euclid(max_y);

        let i = match (end_x.cmp(&mid_x), end_y.cmp(&mid_y)) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => 0,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => 2,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => 3,
            _ => continue,
        };
        sums[i] += 1;
    }
    sums.into_iter().product()
}

#[derive(Clone, Copy, Debug)]
struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

impl Robot {
    fn travel(&mut self, max_x: i64, max_y: i64, num_steps: i64) {
        self.p.0 = (self.p.0 + self.v.0 * num_steps).rem_euclid(max_x);
        self.p.1 = (self.p.1 + self.v.1 * num_steps).rem_euclid(max_y);
    }
}

const RE: &str = r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)";

fn parse_robots(data: &str) -> Vec<Robot> {
    Regex::new(RE)
        .unwrap()
        .captures_iter(data)
        .map(|c| c.extract::<4>())
        .map(|(_, matches)| Robot {
            p: (matches[0].parse().unwrap(), matches[1].parse().unwrap()),
            v: (matches[2].parse().unwrap(), matches[3].parse().unwrap()),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_problem_1() {
        let robots = parse_robots(DATA);
        assert_eq!(12, simulate_robots(&robots, 11, 7));
    }
}
