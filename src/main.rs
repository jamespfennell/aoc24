use std::{collections::HashMap, fs};

fn main() {
    let data: Vec<[i32; 2]> = read_file("data/day1.txt");
    let mut left_column: Vec<i32> = data.iter().map(|row| row[0]).collect();
    left_column.sort();
    let mut right_column: Vec<i32> = data.iter().map(|row| row[1]).collect();
    right_column.sort();

    let mut hist: HashMap<i32, i32> = Default::default();
    for r in &right_column {
        *hist.entry(*r).or_insert(0) += 1;
    }

    let mut similarity = 0_i32;
    for l in &left_column {
        similarity += *l * hist.get(l).copied().unwrap_or(0);
    }
    println!("{:?}", similarity);
}

fn day_1_problem_1() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut columns: [Vec<i32>; 2] = [vec![], vec![]];
    for line in contents.lines() {
        for (index, word) in line.split_ascii_whitespace().enumerate() {
            let i = word
                .parse::<i32>()
                .expect("all words in the file are integers");
            columns
                .get_mut(index)
                .expect("only 2 numbers on each line")
                .push(i);
        }
    }
    columns[0].sort();
    columns[1].sort();
    let mut total = 0_i32;
    for i in 0..columns[1].len() {
        total += (columns[0][i] - columns[1][i]).abs();
    }
    println!("{:?}", total);
}

fn read_file<const N: usize>(path: &str) -> Vec<[i32; N]> {
    let contents = fs::read_to_string(path).expect("could not open file");
    let mut rows: Vec<[i32; N]> = vec![];
    for line in contents.lines() {
        let mut row = [0; N];
        for (i, word) in line.split_ascii_whitespace().enumerate() {
            let cell = match row.get_mut(i) {
                Some(cell) => cell,
                None => break,
            };
            *cell = word
                .parse::<i32>()
                .expect("all words in the file are integers");
        }
        rows.push(row);
    }
    rows
}
