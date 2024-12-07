pub struct File {
    rows: Vec<Vec<i64>>,
}

impl File {
    pub fn rows(&self) -> &[Vec<i64>] {
        &self.rows
    }
    pub fn column<const M: usize>(&self) -> Vec<i64> {
        self.rows.iter().map(|row| row[M]).collect()
    }
}

pub fn read_file(data: &str) -> File {
    let mut rows: Vec<Vec<i64>> = vec![];
    for line in data.lines() {
        let mut row = vec![];
        for word in line.split_ascii_whitespace() {
            let cell = word
                .parse::<i64>()
                .expect("all words in the file are integers");
            row.push(cell);
        }
        rows.push(row);
    }
    File { rows }
}
