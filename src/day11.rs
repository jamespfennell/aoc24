use std::collections::HashMap;

pub fn problem_1(data: &str) -> i64 {
    run(data, 25)
}

pub fn problem_2(data: &str) -> i64 {
    run(data, 75)
}

pub fn run(data: &str, n: usize) -> i64 {
    let mut stones: HashMap<i64, i64> = data
        .split_ascii_whitespace()
        .map(|word| (word.parse::<i64>().unwrap(), 1))
        .collect();
    for _ in 0..n {
        stones = blink_once(&stones);
    }
    stones.values().sum()
}

fn blink_once(stones: &HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut out: HashMap<i64, i64> = Default::default();
    for (&stone, &frequency) in stones {
        let mut insert = |new_stone: i64| {
            *out.entry(new_stone).or_default() += frequency;
        };
        if stone == 0 {
            insert(1);
            continue;
        }
        let l = log_base_10(stone);
        if l % 2 == 0 {
            let d = (10_i64).pow(l / 2);
            insert(stone / d);
            insert(stone % d);
        } else {
            insert(stone * 2024);
        }
    }
    out
}

fn log_base_10(mut n: i64) -> u32 {
    let mut l = 0;
    while n > 0 {
        n /= 10;
        l += 1;
    }
    l
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "125 17";

    #[test]
    fn test_problem_1() {
        assert_eq!(55312, problem_1(DATA));
    }
}
