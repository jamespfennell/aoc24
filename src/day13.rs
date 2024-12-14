use std::i64;

use regex::Regex;

pub fn problem_1(data: &str) -> i64 {
    parse_machines(data, 0)
        .into_iter()
        .filter_map(|m| m.min_price_to_win(100))
        .sum()
}

pub fn problem_2(data: &str) -> i64 {
    parse_machines(data, 10000000000000)
        .into_iter()
        .filter_map(|m| m.min_price_to_win(i64::MAX))
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn min_price_to_win(&self, max_pushes: i64) -> Option<i64> {
        let Machine {
            button_a: (a_0, a_1),
            button_b: (b_0, b_1),
            prize: (p_0, p_1),
        } = *self;
        let det = a_0 * b_1 - a_1 * b_0;
        if det == 0 {
            panic!("linearly dependent vectors - can't solve with current algorithm {self:?}");
        }
        let push_a_times_det = b_1 * p_0 - b_0 * p_1;
        let push_b_times_det = -a_1 * p_0 + a_0 * p_1;
        if push_a_times_det % det != 0 || push_b_times_det % det != 0 {
            // Would require a fractional number of moves
            return None;
        }
        let push_a = push_a_times_det / det;
        let push_b = push_b_times_det / det;
        if push_a < 0 || push_b < 0 {
            // Would require negative moves
            return None;
        }
        if push_a > max_pushes || push_b > max_pushes {
            // Would require more than the max moves
            return None;
        }
        Some(3 * push_a + push_b)
    }
}

const RE: &str = r"Button A: X\+([0-9]+), Y\+([0-9]+)
Button B: X\+([0-9]+), Y\+([0-9]+)
Prize: X=([0-9]+), Y=([0-9]+)";

fn parse_machines(data: &str, prize_increment: i64) -> Vec<Machine> {
    Regex::new(RE)
        .unwrap()
        .captures_iter(data)
        .map(|c| c.extract::<6>())
        .map(|(_, matches)| Machine {
            button_a: (matches[0].parse().unwrap(), matches[1].parse().unwrap()),
            button_b: (matches[2].parse().unwrap(), matches[3].parse().unwrap()),
            prize: (
                matches[4].parse::<i64>().unwrap() + prize_increment,
                matches[5].parse::<i64>().unwrap() + prize_increment,
            ),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1, DATA, 480),
        (test_problem_2_data_1, problem_2, DATA, 875318608908),
    );
}
