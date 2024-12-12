// From day 10
pub struct Neighbors {
    max_r: usize,
    max_c: usize,
    r: usize,
    c: usize,
    next: usize,
}

impl Neighbors {
    pub fn new<T>(grid: &[Vec<T>], r: usize, c: usize) -> Self {
        Self {
            max_r: grid.len(),
            max_c: grid[0].len(),
            r,
            c,
            next: 0,
        }
    }
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
