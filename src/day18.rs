use super::algorithms;
use super::common::Direction;
use super::common::Point;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn problem_1(data: &str) -> i64 {
    let input = parse_input(data);
    solve(&input, 71, 1024).unwrap()
}

pub fn problem_2(data: &str) -> String {
    solve_problem_2(data, 71)
}

fn solve_problem_2(data: &str, width: usize) -> String {
    let input = parse_input(data);
    let mut lower = 0;
    let mut upper = input.len();
    while upper - lower > 1 {
        let t = lower + (upper - lower) / 2;
        println!("current bounds: [{lower},{upper}). trying {t}");
        if solve(&input, width, t).is_none() {
            upper = t;
        } else {
            lower = t;
        }
    }
    let blocker = input[lower];
    format!("{},{}", blocker.1, blocker.0)
}

fn parse_input(data: &str) -> Vec<Point> {
    data.lines()
        .map(|line| {
            let mut cells = line.split(',').map(|cell| cell.parse::<usize>().unwrap());
            let c = cells.next().unwrap();
            let r = cells.next().unwrap();
            Point(r, c)
        })
        .collect()
}

fn solve(corrupted: &[Point], width: usize, steps: usize) -> Option<i64> {
    let corrupted: HashSet<Point> = corrupted.iter().copied().take(steps).collect();

    let mut edges: HashMap<Point, Vec<(Point, i64)>> = Default::default();
    for r in 0..width {
        for c in 0..width {
            let point = Point(r, c);
            if corrupted.contains(&point) {
                continue;
            }
            for direction in Direction::all_directions() {
                let Some(next) = direction.next_checked((r, c), (width, width)) else {
                    continue;
                };
                let next = Point(next.0, next.1);
                if corrupted.contains(&next) {
                    continue;
                }
                edges.entry(point).or_default().push((next, 1));
            }
        }
    }

    let shortest_paths = algorithms::calculate_shortest_paths(edges, Point(0, 0));
    shortest_paths
        .get(&Point(width - 1, width - 1))
        .map(|t| t.cost)
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    fn problem_1_example(data: &str) -> i64 {
        let input = parse_input(data);
        solve(&input, 7, 12).unwrap()
    }

    fn problem_2_example(data: &str) -> String {
        solve_problem_2(data, 7)
    }

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1_example, DATA, 22),
        (test_problem_2_data_1, problem_2_example, DATA, "6,1"),
    );
}
