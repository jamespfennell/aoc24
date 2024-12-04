use std::fs;

pub struct File {
    rows: Vec<Vec<i32>>,
}

impl File {
    pub fn rows(&self) -> &[Vec<i32>] {
        &self.rows
    }
    pub fn column<const M: usize>(&self) -> Vec<i32> {
        self.rows.iter().map(|row| row[M]).collect()
    }
}

pub fn read_file(path: &str) -> File {
    let contents = fs::read_to_string(path).expect("could not open file");
    let mut rows: Vec<Vec<i32>> = vec![];
    for line in contents.lines() {
        let mut row = vec![];
        for word in line.split_ascii_whitespace() {
            let cell = word
                .parse::<i32>()
                .expect("all words in the file are integers");
            row.push(cell);
        }
        rows.push(row);
    }
    File { rows }
}
