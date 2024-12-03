use std::fs;

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
    let mut total = 0_i32;
    for i in 0..columns[1].len() {
        total += (columns[0][i] - columns[1][i]).abs();
    }
    println!("{:?}", total);
}
