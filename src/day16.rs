use super::algorithms::calculate_shortest_paths;
use super::common::Direction;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    i64,
};

pub fn problem_1(data: &str) -> i64 {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let edges = build_edges(&grid);
    let start = Vertex {
        r: grid.len() - 2,
        c: 1,
        direction: Direction::Right,
    };
    assert_eq!(grid[start.r][start.c], 'S');
    let lowest_prices = calculate_shortest_paths(edges, start);
    Direction::all_directions()
        .map(|direction| {
            let end = Vertex {
                r: 1,
                c: grid[0].len() - 2,
                direction,
            };
            assert_eq!(grid[end.r][end.c], 'E');
            lowest_prices.get(&end).unwrap().clone()
        })
        .into_iter()
        .map(|l| l.cost)
        .min()
        .unwrap()
}

pub fn problem_2(data: &str) -> i64 {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let edges = build_edges(&grid);
    let start = Vertex {
        r: grid.len() - 2,
        c: 1,
        direction: Direction::Right,
    };
    assert_eq!(grid[start.r][start.c], 'S');
    let lowest_prices = calculate_shortest_paths(edges, start);
    let ends = Direction::all_directions().map(|direction| {
        let end = Vertex {
            r: 1,
            c: grid[0].len() - 2,
            direction,
        };
        assert_eq!(grid[end.r][end.c], 'E');
        (end.clone(), lowest_prices.get(&end).unwrap().clone())
    });
    let min_price = ends.iter().map(|l| l.1.cost).min().unwrap();

    let mut good_v: HashSet<Vertex> = Default::default();
    let mut pending: Vec<Vertex> = Default::default();
    for (v, end) in ends {
        if end.cost != min_price {
            continue;
        }
        good_v.insert(v);
        for source in end.sources {
            pending.push(source);
        }
    }
    while let Some(v) = pending.pop() {
        if good_v.contains(&v) {
            continue;
        }
        for source in &lowest_prices.get(&v).unwrap().sources {
            pending.push(source.clone());
        }
        good_v.insert(v.clone());
    }

    let good_tiles: HashSet<(usize, usize)> = good_v.into_iter().map(|v| v.pos()).collect();
    good_tiles.len().try_into().unwrap()
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Vertex {
    r: usize,
    c: usize,
    direction: Direction,
}

impl Vertex {
    fn pos(&self) -> (usize, usize) {
        (self.r, self.c)
    }
}

fn build_edges(grid: &[Vec<char>]) -> HashMap<Vertex, Vec<(Vertex, i64)>> {
    let mut edges: HashMap<Vertex, Vec<(Vertex, i64)>> = Default::default();
    for r in 1..grid.len() - 1 {
        for c in 1..grid[0].len() - 1 {
            if grid[r][c] == '#' {
                continue;
            }
            for direction in Direction::all_directions() {
                let this = Vertex { r, c, direction };
                let edges = edges.entry(this).or_default();

                // (1) We can rotate within the same physical position
                for o in direction.orthogonal() {
                    edges.push((Vertex { r, c, direction: o }, 1000));
                }

                // (2) Or we can move to a new square
                let dest = direction.next((r, c));
                if grid[dest.0][dest.1] == '#' {
                    continue;
                }
                edges.push((
                    Vertex {
                        r: dest.0,
                        c: dest.1,
                        direction,
                    },
                    1,
                ));
            }
        }
    }
    edges
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const DATA_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1, DATA_1, 7036),
        (test_problem_1_data_2, problem_1, DATA_2, 11048),
        (test_problem_2_data_1, problem_2, DATA_1, 45),
        (test_problem_2_data_2, problem_2, DATA_2, 64),
    );
}
