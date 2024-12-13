use regex::Regex;

pub fn problem_1(data: &str) -> i64 {
    // 29853 <- too high
    parse_machines(data)
        .into_iter()
        .filter_map(|m| m.min_price_to_win())
        .sum()
}

pub fn problem_2(_data: &str) -> i64 {
    0
}

#[derive(Debug)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn min_price_to_win(&self) -> Option<i64> {
        let mut price: Option<i64> = None;
        // We push button_b up to 100 times - the puzzle says it won't be more
        // In fact the puzzle gives the wrong answer if you allow more than 100.
        for push_b in 0..=100_i64 {
            let Some(remainder_0) = self.prize.0.checked_sub(self.button_b.0 * push_b) else {
                break;
            };
            let Some(remainder_1) = self.prize.1.checked_sub(self.button_b.1 * push_b) else {
                break;
            };
            if (remainder_0 % self.button_a.0 != 0) && (remainder_1 % self.button_a.0 != 0) {
                continue;
            }
            let push_a = remainder_0 / self.button_a.0;
            if push_a != remainder_1 / self.button_a.1 {
                continue;
            }
            if push_a > 100 {
                continue;
            }
            price = Some(push_b + 3 * (remainder_0 / self.button_a.0));
        }
        price
    }
}

const RE: &str = r"Button A: X\+([0-9]+), Y\+([0-9]+)
Button B: X\+([0-9]+), Y\+([0-9]+)
Prize: X=([0-9]+), Y=([0-9]+)";

fn parse_machines(data: &str) -> Vec<Machine> {
    Regex::new(RE)
        .unwrap()
        .captures_iter(data)
        .map(|c| c.extract::<6>())
        .map(|(_, matches)| Machine {
            button_a: (matches[0].parse().unwrap(), matches[1].parse().unwrap()),
            button_b: (matches[2].parse().unwrap(), matches[3].parse().unwrap()),
            prize: (matches[4].parse().unwrap(), matches[5].parse().unwrap()),
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
        (test_problem_2_data_1, problem_2, DATA, 0),
    );
}
