use std::collections::HashMap;

use super::input;

pub fn problem_1(data: &str) -> i64 {
    let data: input::File = input::read_file(data);
    let mut left_column = data.column::<0>();
    left_column.sort();
    let mut right_column: Vec<i64> = data.column::<1>();
    right_column.sort();

    let mut total = 0_i64;
    for i in 0..right_column.len() {
        total += (left_column[i] - right_column[i]).abs();
    }
    total
}

pub fn problem_2(data: &str) -> i64 {
    let data: input::File = input::read_file(data);
    let mut left_column = data.column::<0>();
    left_column.sort();
    let mut right_column: Vec<i64> = data.column::<1>();
    right_column.sort();

    let mut hist: HashMap<i64, i64> = Default::default();
    for r in &right_column {
        *hist.entry(*r).or_insert(0) += 1;
    }

    let mut similarity = 0_i64;
    for l in &left_column {
        similarity += *l * hist.get(l).copied().unwrap_or(0);
    }
    similarity
}
