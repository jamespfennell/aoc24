use std::collections::HashMap;

use super::input;

pub fn problem_1() -> i32 {
    let data: input::File = input::read_file("data/day2.txt");

    data.rows()
        .into_iter()
        .filter(|row| {
            (0..row.len() - 1).all(|i| {
                let diff = (row[i] - row[i + 1]).abs();
                diff >= 1 && diff <= 3 && row[i].cmp(&row[i + 1]) == row[0].cmp(&row[1])
            })
        })
        .count()
        .try_into()
        .unwrap()
}

pub fn problem_2() -> i32 {
    let data: input::File = input::read_file("data/day1.txt");
    let mut left_column = data.column::<0>();
    left_column.sort();
    let mut right_column: Vec<i32> = data.column::<1>();
    right_column.sort();

    let mut hist: HashMap<i32, i32> = Default::default();
    for r in &right_column {
        *hist.entry(*r).or_insert(0) += 1;
    }

    let mut similarity = 0_i32;
    for l in &left_column {
        similarity += *l * hist.get(l).copied().unwrap_or(0);
    }
    similarity
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_problem_1() {
        assert_eq!(341_i32, problem_1());
    }
    #[test]
    fn test_problem_2() {
        assert_eq!(18567089_i32, problem_2());
    }
}
