use std::collections::HashMap;

use crate::{
    algorithms::{self, ShortestPath},
    common::{Direction, Point},
};

pub fn problem_1(data: &str) -> i64 {
    solve_problems(data, 2, 100)
}

pub fn problem_2(data: &str) -> i64 {
    solve_problems(data, 20, 100)
}

fn solve_problems(data: &str, max_cheat: usize, min_saving: i64) -> i64 {
    let input = Input::parse(data);

    let paths = input.solve();

    let mut sum: i64 = 0;
    for r_1 in 1..input.grid.len() - 1 {
        for c_1 in 1..input.grid[0].len() - 1 {
            // we start the cheat at (r_1,c_1)
            let p_1 = Point(r_1, c_1);

            // cheat must start at an reachable space
            let Some(cost_1) = paths.get(&p_1).map(|s| s.cost) else {
                continue;
            };

            let r_2_min = r_1.checked_sub(max_cheat).unwrap_or_default().max(1);
            let r_2_max = (r_1 + max_cheat).min(input.grid.len() - 2);
            let c_2_min = c_1.checked_sub(max_cheat).unwrap_or_default().max(1);
            let c_2_max = (c_1 + max_cheat).min(input.grid[0].len() - 2);

            for r_2 in r_2_min..=r_2_max {
                for c_2 in c_2_min..=c_2_max {
                    let p_2 = Point(r_2, c_2);
                    let d = p_1.distance(p_2);
                    if d > max_cheat {
                        continue;
                    }
                    // cheat must end at an reachable space
                    let Some(cost_2) = paths.get(&p_2).map(|s| s.cost) else {
                        continue;
                    };
                    if cost_1 + (d as i64) + min_saving <= cost_2 {
                        sum += 1;
                    }
                }
            }
        }
    }
    sum
}

#[derive(Debug)]
struct Input {
    start: Point,
    grid: Vec<Vec<bool>>,
}

impl Input {
    fn parse(data: &str) -> Self {
        let mut start = None;
        let grid = data
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c, ch)| {
                        match ch {
                            '#' => return false,
                            'S' => {
                                start = Some(Point(r, c));
                            }
                            _ => {}
                        };
                        true
                    })
                    .collect()
            })
            .collect();
        Input {
            start: start.unwrap(),
            grid,
        }
    }

    fn solve(&self) -> HashMap<Point, ShortestPath<Point>> {
        let mut edges: HashMap<Point, Vec<(Point, i64)>> = Default::default();
        for r in 1..self.grid.len() - 1 {
            for c in 1..self.grid[0].len() - 1 {
                for direction in Direction::all_directions() {
                    let (r_dest, c_dest) = direction.next((r, c));
                    if self.grid[r][c] && self.grid[r_dest][c_dest] {
                        edges
                            .entry(Point(r, c))
                            .or_default()
                            .push((Point(r_dest, c_dest), 1));
                    }
                }
            }
        }
        algorithms::calculate_shortest_paths(edges, self.start)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    fn problem_1_test(data: &str) -> i64 {
        solve_problems(data, 2, 1)
    }

    fn problem_2_test(data: &str) -> i64 {
        solve_problems(data, 20, 50)
    }

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1_test, DATA, 44),
        (test_problem_2_data_1, problem_2_test, DATA, 285),
    );
}
