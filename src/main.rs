use std::{collections::HashMap, fs};

fn main() {
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


    let mut rhs: HashMap<i32, i32> = Default::default();
    for r in &columns[1] {
        *rhs.entry(*r).or_insert(0) += 1;
    }

    let mut similarity = 0_i32;
    for l in &columns[0] {
        similarity += *l * rhs.get(l).copied().unwrap_or(0);
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
