use std::collections::HashMap;

use crate::{
    algorithms::{self, ShortestPath},
    common::{Direction, Point},
};

pub fn problem_1(data: &str) -> i64 {
    solve_problem_1(data, 100)
}

fn solve_problem_1(data: &str, min_saving: i64) -> i64 {
    let input = Input::parse(data);

    let paths = input.solve();

    let mut sum = 0;
    for r_1 in 1..input.grid.len() - 1 {
        for c_1 in 1..input.grid[0].len() - 1 {
            // we start the cheat at (r_1,c_1)

            // cheat must start at an reachable space
            let Some(cost_1) = paths.get(&Point(r_1, c_1)).map(|s| s.cost) else {
                continue;
            };

            for d_1 in Direction::all_directions() {
                let (r_2, c_2) = d_1.next((r_1, c_1));
                // cheat must visit a closed space
                if input.grid[r_2][c_2] {
                    continue;
                }
                if r_2 == 0
                    || r_2 == input.grid.len() - 1
                    || c_2 == 0
                    || c_2 == input.grid[0].len() - 1
                {
                    continue;
                }
                for d_2 in Direction::all_directions() {
                    let (r_3, c_3) = d_2.next((r_2, c_2));
                    // cheat must end at an reachable space
                    let Some(cost_2) = paths.get(&Point(r_3, c_3)).map(|s| s.cost) else {
                        continue;
                    };
                    if cost_1 + 2 + min_saving <= cost_2 {
                        sum += 1;
                    }
                }
            }
        }
    }
    // 1484 too high
    sum
}

pub fn problem_2(_data: &str) -> i64 {
    0
}

#[derive(Debug)]
struct Input {
    start: Point,
    end: Point,
    grid: Vec<Vec<bool>>,
}

impl Input {
    fn parse(data: &str) -> Self {
        let mut start = None;
        let mut end = None;
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
                            'E' => {
                                end = Some(Point(r, c));
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
            end: end.unwrap(),
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
        solve_problem_1(data, 1)
    }

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1_test, DATA, 44),
        (test_problem_2_data_1, problem_2, DATA, 0),
    );
}
