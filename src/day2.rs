use super::input;

pub fn problem_1(data: &str) -> i64 {
    let data: input::File = input::read_file(data);

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

pub fn problem_2(data: &str) -> i64 {
    let data: input::File = input::read_file(data);

    data.rows()
        .into_iter()
        .filter(|row| {
            (0..row.len() + 1).any(|element_to_skip| {
                // If element_to_skip == row.len() then no element is skipped.
                // This ensures we still accept a row that is valid with no skipped elements.
                let row: Vec<i64> = Skipper::new(row, element_to_skip).collect();
                (0..row.len() - 1).all(|i| {
                    let diff = (row[i] - row[i + 1]).abs();
                    diff >= 1 && diff <= 3 && row[i].cmp(&row[i + 1]) == row[0].cmp(&row[1])
                })
            })
        })
        .count()
        .try_into()
        .unwrap()
}

struct Skipper<'a> {
    row: &'a [i64],
    skip: usize,
    current: usize,
}

impl<'a> Skipper<'a> {
    fn new(row: &'a [i64], skip: usize) -> Self {
        Self {
            row,
            skip,
            current: 0,
        }
    }
}

impl<'a> Iterator for Skipper<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.skip {
            self.current += 1;
        }
        let elem = self.row.get(self.current).copied();
        self.current += 1;
        elem
    }
}
