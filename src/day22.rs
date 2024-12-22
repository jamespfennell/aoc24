use std::{collections::HashMap, i64};

pub fn problem_1(data: &str) -> i64 {
    data.lines()
        .map(|l| {
            let mut n: i64 = l.parse().unwrap();
            for _ in 0..2000 {
                n = evolve_secret(n);
            }
            n
        })
        .sum()
}

pub fn problem_2(data: &str) -> i64 {
    let maps: Vec<HashMap<[i8; 4], i64>> = data
        .lines()
        .map(|l| {
            let mut n: i64 = l.parse().unwrap();
            let mut diffs: Vec<i8> = vec![];
            let mut prices: Vec<i64> = vec![];
            for _ in 0..2000 {
                let next_n = evolve_secret(n);
                prices.push(next_n % 10);
                diffs.push((next_n % 10 - n % 10).try_into().unwrap());
                n = next_n;
            }

            let mut m: HashMap<[i8; 4], i64> = Default::default();
            for i in 3..2000 {
                let key = [diffs[i - 3], diffs[i - 2], diffs[i - 1], diffs[i]];
                m.entry(key).or_insert(prices[i]);
            }
            m
        })
        .collect();

    let mut merged: HashMap<[i8; 4], i64> = Default::default();
    for m in maps {
        for (k, v) in m {
            *merged.entry(k).or_default() += v;
        }
    }

    let mut max = 0;
    for (_, v) in merged {
        max = max.max(v);
    }
    max
}

fn evolve_secret(n: i64) -> i64 {
    let n = ((n * 64) ^ n) % 16777216;
    let n = ((n / 32) ^ n) % 16777216;
    let n = ((n * 2048) ^ n) % 16777216;
    n
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA_1: &str = "1
10
100
2024";

    const DATA_2: &str = "1
2
3
2024";

    #[test]
    fn test_evolve_secret() {
        let secrets = "15887950
16495136
527345
704524
1553684
12683156
11100544
12249484
7753432
5908254";

        let secrets: Vec<i64> = secrets.lines().map(|l| l.parse().unwrap()).collect();
        let mut n = 123;
        for secret in secrets {
            let new_n = evolve_secret(n);
            assert_eq!(secret, new_n);
            n = new_n;
        }
    }

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1, DATA_1, 37327623),
        (test_problem_2_data_2, problem_2, DATA_2, 23),
    );
}
