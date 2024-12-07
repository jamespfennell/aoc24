pub fn problem_1(data: &str) -> i64 {
    problem(data, false)
}

pub fn problem_2(data: &str) -> i64 {
    problem(data, true)
}

pub fn problem(data: &str, do_and_dont: bool) -> i64 {
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
                let r: i64 = (next as usize - '0' as usize)
                    .try_into()
                    .expect("digit fits in i64");
                FirstArg(current * 10 + r)
            }
            (FirstArg(current), ',') => SecondArg(current, 0),
            (SecondArg(first, current), '0'..='9') => {
                let r: i64 = (next as usize - '0' as usize)
                    .try_into()
                    .expect("digit fits in i64");
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
    FirstArg(i64),
    SecondArg(i64, i64),
}

impl Default for State {
    fn default() -> Self {
        State::FunctionName(String::new())
    }
}
