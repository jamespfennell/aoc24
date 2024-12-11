pub fn problem_1(data: &str) -> i64 {
    let mut stones: Vec<i64> = data
        .split_ascii_whitespace()
        .map(|word| word.parse::<i64>().unwrap())
        .collect();
    for _ in 0..25 {
        stones = blink_once(&stones);
    }
    stones.len().try_into().unwrap()
}

pub fn problem_2(_data: &str) -> i64 {
    0
}

fn blink_once(stones: &[i64]) -> Vec<i64> {
    let mut out = vec![];
    for &stone in stones {
        if stone == 0 {
            out.push(1);
            continue;
        }
        let l = log_base_10(stone);
        if l % 2 == 0 {
            let d = (10_i64).pow(l / 2);
            out.push(stone / d);
            out.push(stone % d);
        } else {
            out.push(stone * 2024);
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

    #[test]
    fn test_problem_2() {
        assert_eq!(0, problem_2(DATA));
    }
}
