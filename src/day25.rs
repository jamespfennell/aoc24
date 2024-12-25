pub fn problem_1(data: &str) -> i64 {
    let (locks, keys) = parse_input(data);

    let mut sum = 0;
    for &lock in &locks {
        for &key in &keys {
            let mut max = 0;
            for i in 0..5 {
                max = max.max(lock[i] + key[i]);
            }
            if max <= 5 {
                sum += 1;
            }
        }
    }
    sum
}

pub fn problem_2(_data: &str) -> i64 {
    0
}

pub fn parse_input(data: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let mut locks = vec![];
    let mut keys = vec![];
    let mut lines = data.lines();
    while let Some(header) = lines.next() {
        let mut thing = [0_u8; 5];
        for _ in 0..5 {
            for (c, ch) in lines.next().unwrap().chars().enumerate() {
                if ch == '#' {
                    thing[c] = thing[c] + 1;
                }
            }
        }
        let target = if header == "#####" {
            // lock
            &mut locks
        } else {
            &mut keys
        };
        target.push(thing);
        lines.next();
        lines.next();
    }
    (locks, keys)
}
#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1, DATA, 3),
        (test_problem_2_data_1, problem_2, DATA, 0),
    );
}
