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

fn evolve_secret(n: i64) -> i64 {
    let n = ((n * 64) ^ n) % 16777216;
    let n = ((n / 32) ^ n) % 16777216;
    let n = ((n * 2048) ^ n) % 16777216;
    n
}

/*
Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.

Each step of the above process involves mixing and pruning:

To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number. Then, the secret number becomes the result of that operation. (If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.)
To prune the secret number, calculate the value of the secret number modulo 16777216. Then, the secret number becomes the result of that operation. (If the secret number is 100000000 and you were to prune the secret number, the secret number would become 16113920.)
 */

pub fn problem_2(_data: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "1
10
100
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
        (test_problem_1_data_1, problem_1, DATA, 37327623),
        (test_problem_2_data_1, problem_2, DATA, 0),
    );
}
