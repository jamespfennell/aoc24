use std::fs;

pub fn problem_1() -> i32 {
    problem(false)
}

pub fn problem_2() -> i32 {
    problem(true)
}

pub fn problem(do_and_dont: bool) -> i32 {
    let data = fs::read_to_string("data/day3.txt").expect("expected file to exist");
    let mut state: State = Default::default();
    let mut sum = 0;
    let mut enabled = true;
    for next in data.chars() {
        use State::*;
        state = match (state, next) {
            (FunctionName(s), '(') => {
                if s.ends_with("do") {
                    NoArg("do")
                } else if s.ends_with("don't") {
                    NoArg("don't")
                } else if s.ends_with("mul") {
                    FirstArg(0)
                } else {
                    Default::default()
                }
            }
            (FunctionName(mut s), c) => {
                s.push(c);
                FunctionName(s)
            }
            (NoArg(fn_name), ')') => {
                if fn_name == "do" {
                    enabled = true;
                }
                if fn_name == "don't" && do_and_dont {
                    enabled = false;
                }
                Default::default()
            }
            (FirstArg(current), '0'..='9') => {
                let r: i32 = (next as usize - '0' as usize)
                    .try_into()
                    .expect("digit fits in i32");
                FirstArg(current * 10 + r)
            }
            (FirstArg(current), ',') => SecondArg(current, 0),
            (SecondArg(first, current), '0'..='9') => {
                let r: i32 = (next as usize - '0' as usize)
                    .try_into()
                    .expect("digit fits in i32");
                SecondArg(first, current * 10 + r)
            }
            (SecondArg(first, second), ')') => {
                if enabled {
                    sum += first * second;
                }
                Default::default()
            }
            (_, c) => FunctionName(c.into()),
        };
    }
    sum
}

enum State {
    FunctionName(String),
    NoArg(&'static str),
    FirstArg(i32),
    SecondArg(i32, i32),
}

impl Default for State {
    fn default() -> Self {
        State::FunctionName(String::new())
    }
}
