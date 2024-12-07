pub fn problem_1(data: &str) -> i64 {
    solve(data, false)
}

pub fn problem_2(data: &str) -> i64 {
    solve(data, true)
}

pub fn solve(data: &str, allow_concat_operator: bool) -> i64 {
    read(data)
        .into_iter()
        .filter(|row| is_possibly_valid(row[0], row[1], &row[2..], allow_concat_operator))
        .map(|row| row[0])
        .sum()
}

fn is_possibly_valid(target: i64, current: i64, tail: &[i64], allow_concat_operator: bool) -> bool {
    if tail.is_empty() {
        return target == current;
    }
    // * and + and || can only make numbers bigger, so if current is already bigger than target
    // it is never going to match. Short circuit in this case.
    if current > target {
        return false;
    }
    // Try a +
    if is_possibly_valid(target, current + tail[0], &tail[1..], allow_concat_operator) {
        return true;
    }
    // Try a *
    if is_possibly_valid(target, current * tail[0], &tail[1..], allow_concat_operator) {
        return true;
    }
    // Try a ||
    if !allow_concat_operator {
        return false;
    }
    is_possibly_valid(
        target,
        current * round_up_to_power_of_10(tail[0]) + tail[0],
        &tail[1..],
        allow_concat_operator,
    )
}

fn round_up_to_power_of_10(mut n: i64) -> i64 {
    let mut l = 1;
    while n > 0 {
        l *= 10;
        n = n / 10
    }
    l
}

fn read(data: &str) -> Vec<Vec<i64>> {
    let mut rows: Vec<Vec<i64>> = vec![];
    for line in data.lines() {
        let mut row = vec![];
        for word in line.split_ascii_whitespace() {
            let word = word.strip_suffix(':').unwrap_or(word);
            let cell = word
                .parse::<i64>()
                .expect(&format!["can't parse {} as integer", word]);
            row.push(cell);
        }
        rows.push(row);
    }
    rows
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_problem_1() {
        let data = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(3749, problem_1(data));
    }

    #[test]
    fn test_problem_2() {
        let data = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(11387, problem_2(data));
    }
}
