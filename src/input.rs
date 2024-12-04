use std::fs;

pub struct File<const N: usize> {
    rows: Vec<[i32; N]>,
}

impl<const N: usize> File<N> {
    pub fn column<const M: usize>(&self) -> Vec<i32> {
        self.rows.iter().map(|row| row[M]).collect()
    }
}

pub fn read_file<const N: usize>(path: &str) -> File<N> {
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
    File { rows }
}
