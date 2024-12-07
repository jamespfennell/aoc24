use std::collections::HashSet;

pub fn problem_1(data: &str) -> i32 {
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
        has_obstable: &has_obstacle,
    };
    let visited: HashSet<(usize, usize)> = position.into_iter().collect();
    visited.len().try_into().unwrap()
}

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
    has_obstable: &'a [Vec<bool>],
}

impl<'a> Iterator for Position<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // If we're out of bounds, the iteration is over.
        if self.r >= self.has_obstable.len() || self.c >= self.has_obstable[0].len() {
            return None;
        }
        let res = (self.r, self.c);
        loop {
            let mut next_or = self.direction.next(self.r, self.c);
            if let Some(next) = next_or {
                if next.0 >= self.has_obstable.len() || next.1 >= self.has_obstable[0].len() {
                    next_or = None;
                }
            }
            let next = match next_or {
                None => {
                    // End the iteration.
                    self.r = usize::MAX;
                    self.c = usize::MAX;
                    break;
                }
                Some(next) => next,
            };
            if !self.has_obstable[next.0][next.1] {
                self.r = next.0;
                self.c = next.1;
                break;
            }
            self.direction = self.direction.turn();
        }
        Some(res)
    }
}

pub fn problem_2(_data: &str) -> i32 {
    0
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
}
