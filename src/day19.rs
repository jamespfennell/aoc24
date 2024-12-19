pub fn problem_1(data: &str) -> i64 {
    // for each character, see if we can make it to this character
    let input = Input::parse(data);
    let mut sum = 0_i64;
    for &pattern in &input.patterns {
        let mut reachable = vec![false; pattern.len()];
        for i in 0..pattern.len() {
            if i > 0 && !reachable[i - 1] {
                continue;
            }
            let pattern = &pattern[i..];
            for &towel in &input.towels {
                if pattern.starts_with(towel) {
                    reachable[i + towel.len() - 1] = true;
                }
            }
        }
        if reachable[pattern.len() - 1] {
            sum += 1;
        }
    }
    sum
}

pub fn problem_2(_data: &str) -> i64 {
    0
}

#[derive(Debug)]
struct Input<'a> {
    towels: Vec<&'a str>,
    patterns: Vec<&'a str>,
}

impl<'a> Input<'a> {
    fn parse(data: &'a str) -> Input<'a> {
        let mut lines = data.lines();
        let towels: Vec<&'a str> = {
            let mut t: Vec<&'a str> = lines.next().unwrap().split(", ").collect();
            t.sort();
            t
        };
        lines.next();
        let patterns: Vec<&'a str> = lines.collect();
        Input { towels, patterns }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1, DATA, 6),
        (test_problem_2_data_1, problem_2, DATA, 0),
    );
}
