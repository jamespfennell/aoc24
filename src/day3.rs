use std::fs;

pub fn problem_1() -> i32 {
    let data = fs::read_to_string("data/day3.txt").expect("expected file to exist");
    let mut state: State = Default::default();
    let mut sum = 0;
    for c in data.chars() {
        let (new_state, pair) = state.consume(c);
        if let Some((lhs, rhs)) = pair {
            sum += lhs * rhs;
        }
        state = new_state
    }
    sum
}

#[derive(Clone, Copy)]
enum State {
    ExpectPrefix(char),
    ExpectFirstNumber(i32),
    ExpectSecondNumber(i32, i32),
}

impl Default for State {
    fn default() -> Self {
        State::ExpectPrefix('m')
    }
}

impl State {
    fn consume(self, next: char) -> (Self, Option<(i32, i32)>) {
        use State::*;
        let n = match (self, next) {
            (ExpectPrefix('m'), 'm') => ExpectPrefix('u'),
            (ExpectPrefix('u'), 'u') => ExpectPrefix('l'),
            (ExpectPrefix('l'), 'l') => ExpectPrefix('('),
            (ExpectPrefix('('), '(') => ExpectFirstNumber(0),
            // same for other digits, probably a fancy way to do this
            (ExpectFirstNumber(current), '0'..='9') => {
                let r: i32 = (next as usize - '0' as usize)
                    .try_into()
                    .expect("digit fits in i32");
                ExpectFirstNumber(current * 10 + r)
            }
            (ExpectFirstNumber(current), ',') => ExpectSecondNumber(current, 0),
            (ExpectSecondNumber(first, current), '0'..='9') => {
                let r: i32 = (next as usize - '0' as usize)
                    .try_into()
                    .expect("digit fits in i32");
                ExpectSecondNumber(first, current * 10 + r)
            }
            (ExpectSecondNumber(first, second), ')') => {
                return (Default::default(), Some((first, second)));
            }
            _ => Default::default(),
        };
        (n, None)
    }
}

pub fn problem_2() -> i32 {
    0
}
