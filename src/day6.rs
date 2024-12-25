use std::collections::HashMap;
use std::collections::HashSet;

pub fn problem_1(data: &str) -> i64 {
    let has_obstacle: Vec<Vec<bool>> = data
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect();
    let start = data
        .lines()
        .enumerate()
        .filter_map(|(r, line)| line.find('^').map(|c| (r, c)))
        .next()
        .expect("expect to find the starting position");
    let position = Position {
        r: start.0,
        c: start.1,
        direction: Direction::Up,
        has_obstacle: &has_obstacle,
    };
    let visited: HashSet<(usize, usize)> =
        position.into_iter().map(|step| (step.r, step.c)).collect();
    visited.len().try_into().unwrap()
}

pub fn problem_2(data: &str) -> i64 {
    let mut has_obstacle: Vec<Vec<bool>> = data
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect();
    let start = data
        .lines()
        .enumerate()
        .filter_map(|(r, line)| line.find('^').map(|c| (r, c)))
        .next()
        .expect("expect to find the starting position");
    // All squares that can be blocked
    let path: Vec<Step> = {
        let position = Position {
            r: start.0,
            c: start.1,
            direction: Direction::Up,
            has_obstacle: &has_obstacle,
        };
        position.into_iter().collect()
    };
    let mut blocked: HashSet<(usize, usize)> = Default::default();
    let mut count = 0;
    for i in 1..path.len() {
        let start = path[i - 1];
        let blockable = path[i];
        if blocked.contains(&(blockable.r, blockable.c)) {
            continue;
        }
        blocked.insert((blockable.r, blockable.c));
        has_obstacle[blockable.r][blockable.c] = true;
        if has_infnite_loop(&has_obstacle, start) {
            count += 1;
        }
        has_obstacle[blockable.r][blockable.c] = false;
    }
    count
}

fn has_infnite_loop(has_obstacle: &[Vec<bool>], start: Step) -> bool {
    // Key = visited squares
    // Value = number of times visited. If we visit 5 or more times we are guaranteed
    // to be on the same path.
    let mut visited: HashMap<Step, u8> = Default::default();
    let position = Position {
        r: start.r,
        c: start.c,
        direction: start.direction,
        has_obstacle,
    };
    for step in position {
        let visits = visited.entry(step).or_default();
        if *visits == 1 {
            return true;
        }
        *visits = *visits + 1;
    }
    false
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
    fn next(&self, r: usize, c: usize) -> Option<(usize, usize)> {
        use Direction::*;
        match self {
            Up => {
                if r == 0 {
                    None
                } else {
                    Some((r - 1, c))
                }
            }
            Down => Some((r + 1, c)),
            Left => {
                if c == 0 {
                    None
                } else {
                    Some((r, c - 1))
                }
            }
            Right => Some((r, c + 1)),
        }
    }
}

struct Position<'a> {
    r: usize,
    c: usize,
    direction: Direction,
    has_obstacle: &'a [Vec<bool>],
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Step {
    // Row coordinate of the box we're leaving
    r: usize,
    // Column coordinate of the box we're leaving
    c: usize,
    // Direction we entered through
    direction: Direction,
}

impl<'a> Iterator for Position<'a> {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        // If we're out of bounds, the iteration is over.
        if self.r >= self.has_obstacle.len() || self.c >= self.has_obstacle[0].len() {
            return None;
        }
        loop {
            let mut next_or = self.direction.next(self.r, self.c);
            if let Some(next) = next_or {
                if next.0 >= self.has_obstacle.len() || next.1 >= self.has_obstacle[0].len() {
                    next_or = None;
                }
            }
            let next = match next_or {
                None => {
                    // End the iteration.
                    let step = Step {
                        r: self.r,
                        c: self.c,
                        direction: self.direction,
                    };
                    self.r = usize::MAX;
                    self.c = usize::MAX;
                    return Some(step);
                }
                Some(next) => next,
            };
            if !self.has_obstacle[next.0][next.1] {
                let step = Step {
                    r: self.r,
                    c: self.c,
                    direction: self.direction,
                };
                self.r = next.0;
                self.c = next.1;
                return Some(step);
            }
            self.direction = self.direction.turn();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_problem_1() {
        let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(41, problem_1(data));
    }

    #[test]
    fn test_problem_2() {
        let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(6, problem_2(data));
    }
}
