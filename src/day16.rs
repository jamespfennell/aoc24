use std::{collections::HashMap, hash::Hash, i64};

pub fn problem_1(data: &str) -> i64 {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let edges = build_edges(&grid);
    let start = Vertex {
        r: grid.len() - 2,
        c: 1,
        direction: Direction::Right,
    };
    assert_eq!(grid[start.r][start.c], 'S');
    let lowest_prices = calculate_lowest_prices(edges, start);
    *Direction::all_directions()
        .map(|direction| {
            let end = Vertex {
                r: 1,
                c: grid[0].len() - 2,
                direction,
            };
            assert_eq!(grid[end.r][end.c], 'E');
            lowest_prices.get(&end).unwrap()
        })
        .into_iter()
        .min()
        .unwrap()
}

pub fn problem_2(_data: &str) -> i64 {
    0
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Vertex {
    r: usize,
    c: usize,
    direction: Direction,
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

fn calculate_lowest_prices<V: Eq + Hash + Clone>(
    edges: HashMap<V, Vec<(V, i64)>>,
    start: V,
) -> HashMap<V, i64> {
    let mut prices: HashMap<V, i64> = Default::default();
    let mut pending: HashMap<V, i64> = Default::default();
    pending.insert(start, 0);
    while let Some((v, price)) = find_smallest(&pending) {
        pending.remove(&v).unwrap();
        if let Some(edges) = edges.get(&v) {
            for (dest, cost) in edges {
                // We've already found the cheapest price for this dest vertex
                if prices.contains_key(dest) {
                    continue;
                }
                let candidate = price + cost;
                let current = pending.entry(dest.clone()).or_insert(candidate);
                if candidate < *current {
                    *current = candidate;
                }
            }
        }
        prices.insert(v, price);
    }
    prices
}

// TODO: we could use a heap to avoid paying O(n) to find the smallest value.
fn find_smallest<V: Clone>(pending: &HashMap<V, i64>) -> Option<(V, i64)> {
    let mut v_or = None;
    for (k, v) in pending {
        let smaller = match v_or {
            None => true,
            Some((_, v_other)) => *v < v_other,
        };
        if smaller {
            v_or = Some((k, *v));
        }
    }
    v_or.map(|(v, c)| (v.clone(), c))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self, (r, c): (usize, usize)) -> (usize, usize) {
        use Direction::*;
        match self {
            Up => (r - 1, c),
            Down => (r + 1, c),
            Left => (r, c - 1),
            Right => (r, c + 1),
        }
    }
    // mark used.
    fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
    fn orthogonal(&self) -> [Self; 2] {
        use Direction::*;
        match self {
            Up | Down => [Left, Right],
            Left | Right => [Up, Down],
        }
    }
    fn all_directions() -> [Self; 4] {
        [
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Up,
        ]
    }
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
        (test_problem_2_data_1, problem_2, DATA_1, 0),
    );
}
