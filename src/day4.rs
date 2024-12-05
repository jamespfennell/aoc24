use std::{collections::HashSet, fs};

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const SAMX: [char; 4] = ['S', 'A', 'M', 'X'];

pub fn problem_1() -> i32 {
    let data = fs::read_to_string("data/day4.txt").expect("expected file to exist");
    let lines: Vec<Vec<char>> = data
        .lines()
        .map(|l| {
            let l: Vec<char> = l.chars().collect();
            l
        })
        .collect();
    let mut num_matches = 0;
    for mode in [
        Mode::DiagonalDown,
        Mode::DiagonalUp,
        Mode::Horizontal,
        Mode::Vertical,
    ] {
        let iter = Iter {
            mode,
            lines: &lines,
            x: 0,
            y: 0,
        };
        num_matches += iter.filter(|i| *i == XMAS || *i == SAMX).count();
    }
    num_matches.try_into().unwrap()
}

struct Iter<'a> {
    mode: Mode,
    lines: &'a [Vec<char>],
    x: usize,
    y: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = [char; 4];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.x >= self.lines.len() && self.y >= self.lines[0].len() {
                return None;
            }
            let mut res = [' '; 4];
            for (i, (x, y)) in self.mode.coords((self.x, self.y)).into_iter().enumerate() {
                if x >= self.lines.len() {
                    self.x = 0;
                    self.y += 1;
                    break;
                }
                if y >= self.lines[x].len() {
                    self.x = self.lines.len();
                    self.y = self.lines[0].len();
                    break;
                }
                res[i] = self.lines[x][y];
            }
            if res[3] != ' ' {
                self.x += 1;
                return Some(res);
            }
        }
    }
}

enum Mode {
    Horizontal,
    Vertical,
    DiagonalDown,
    DiagonalUp,
}

impl Mode {
    fn coords(&self, (x, y): (usize, usize)) -> [(usize, usize); 4] {
        match self {
            Mode::Horizontal => [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Mode::Vertical => [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Mode::DiagonalDown => [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)],
            Mode::DiagonalUp => [(x + 3, y), (x + 2, y + 1), (x + 1, y + 2), (x, y + 3)],
        }
    }
}

pub fn problem_2() -> i32 {
    let data = fs::read_to_string("data/day4.txt").expect("expected file to exist");
    problem_2_with_input(&data)
}

pub fn problem_2_with_input(data: &str) -> i32 {
    let lines: Vec<Vec<char>> = data
        .lines()
        .map(|l| {
            let l: Vec<char> = l.chars().collect();
            l
        })
        .collect();
    let mut hits = 0;
    let allowed_corners: HashSet<[[char; 2]; 2]> = [
        [['M', 'M'], ['S', 'S']],
        [['M', 'S'], ['M', 'S']],
        [['S', 'M'], ['S', 'M']],
        [['S', 'S'], ['M', 'M']],
    ]
    .into_iter()
    .collect();
    for r in 0..lines.len() - 2 {
        for c in 0..lines[0].len() - 2 {
            if lines[r + 1][c + 1] != 'A' {
                continue;
            }
            let corners = [
                [lines[r][c], lines[r][c + 2]],
                [lines[r + 2][c], lines[r + 2][c + 2]],
            ];
            if allowed_corners.contains(&corners) {
                hits += 1;
            }
        }
    }
    // 939 too low
    // 1936 too high
    hits
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_problem_2() {
        let data = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!(9, problem_2_with_input(&data))
    }

    #[test]
    fn test_iter_diagonal() {
        let input = vec![
            vec!['a', 'b', 'c', 'd', 'e'],
            vec!['f', 'g', 'h', 'i', 'j'],
            vec!['k', 'l', 'm', 'n', 'o'],
            vec!['p', 'q', 'r', 's', 't'],
            vec!['u', 'v', 'w', 'x', 'y'],
        ];
        let iter = Iter {
            mode: Mode::DiagonalDown,
            lines: &input,
            x: 0,
            y: 0,
        };
        let got: Vec<[char; 4]> = iter.collect();
        let want = vec![
            ['a', 'g', 'm', 's'],
            ['f', 'l', 'r', 'x'],
            ['b', 'h', 'n', 't'],
            ['g', 'm', 's', 'y'],
        ];
        assert_eq!(got, want);
    }

    #[test]
    fn test_iter_horizontal() {
        let input = vec![
            vec!['a', 'b', 'c', 'd', 'e'],
            vec!['f', 'g', 'h', 'i', 'j'],
            vec!['k', 'l', 'm', 'n', 'o'],
            vec!['p', 'q', 'r', 's', 't'],
            vec!['u', 'v', 'w', 'x', 'y'],
        ];
        let iter = Iter {
            mode: Mode::Horizontal,
            lines: &input,
            x: 0,
            y: 0,
        };
        let got: Vec<[char; 4]> = iter.collect();
        let want = vec![
            ['a', 'f', 'k', 'p'],
            ['f', 'k', 'p', 'u'],
            ['b', 'g', 'l', 'q'],
            ['g', 'l', 'q', 'v'],
            ['c', 'h', 'm', 'r'],
            ['h', 'm', 'r', 'w'],
            ['d', 'i', 'n', 's'],
            ['i', 'n', 's', 'x'],
            ['e', 'j', 'o', 't'],
            ['j', 'o', 't', 'y'],
        ];
        assert_eq!(got, want);
    }

    #[test]
    fn test_iter_vertical() {
        let input = vec![
            vec!['a', 'b', 'c', 'd', 'e'],
            vec!['f', 'g', 'h', 'i', 'j'],
            vec!['k', 'l', 'm', 'n', 'o'],
            vec!['p', 'q', 'r', 's', 't'],
            vec!['u', 'v', 'w', 'x', 'y'],
        ];
        let iter = Iter {
            mode: Mode::Vertical,
            lines: &input,
            x: 0,
            y: 0,
        };
        let got: Vec<[char; 4]> = iter.collect();
        let want = vec![
            ['a', 'b', 'c', 'd'],
            ['f', 'g', 'h', 'i'],
            ['k', 'l', 'm', 'n'],
            ['p', 'q', 'r', 's'],
            ['u', 'v', 'w', 'x'],
            ['b', 'c', 'd', 'e'],
            ['g', 'h', 'i', 'j'],
            ['l', 'm', 'n', 'o'],
            ['q', 'r', 's', 't'],
            ['v', 'w', 'x', 'y'],
        ];
        assert_eq!(got, want);
    }
}
