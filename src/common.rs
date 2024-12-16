#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn next(&self, (r, c): (usize, usize)) -> (usize, usize) {
        use Direction::*;
        match self {
            Up => (r - 1, c),
            Down => (r + 1, c),
            Left => (r, c - 1),
            Right => (r, c + 1),
        }
    }
    #[allow(unused)]
    pub fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
    pub fn orthogonal(&self) -> [Self; 2] {
        use Direction::*;
        match self {
            Up | Down => [Left, Right],
            Left | Right => [Up, Down],
        }
    }
    pub fn all_directions() -> [Self; 4] {
        [
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Up,
        ]
    }
}
