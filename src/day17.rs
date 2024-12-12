pub fn problem_1(_data: &str) -> i64 {
    0
}

pub fn problem_2(_data: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "";

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1, DATA, 0),
        (test_problem_2_data_1, problem_2, DATA, 0),
    );
}
