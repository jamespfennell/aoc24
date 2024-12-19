pub fn problem_1(data: &str) -> i64 {
    let input = Input::parse(data);
    calculate_reachability(&input)
        .filter(|s| *s > 0)
        .count()
        .try_into()
        .unwrap()
}

pub fn problem_2(data: &str) -> i64 {
    let input = Input::parse(data);
    calculate_reachability(&input).sum()
}

fn calculate_reachability<'a>(input: &'a Input<'a>) -> impl Iterator<Item = i64> + 'a {
    input.patterns.iter().map(|pattern| {
        let mut reachable = vec![0_i64; pattern.len()];
        for i in 0..pattern.len() {
            let num_ways_in = if i > 0 { reachable[i - 1] } else { 1 };
            if num_ways_in == 0 {
                continue;
            }
            let pattern = &pattern[i..];
            for &towel in &input.towels {
                if pattern.starts_with(towel) {
                    reachable[i + towel.len() - 1] += num_ways_in;
                }
            }
        }
        reachable[pattern.len() - 1]
    })
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
        (test_problem_2_data_1, problem_2, DATA, 16),
    );
}
