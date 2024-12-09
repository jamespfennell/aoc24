pub fn problem_1(data: &str) -> i64 {
    let mut v: Vec<i32> = vec![];
    let mut file_id = 0;
    let alternator = Alternator { b: true };
    for (c, is_file) in data.chars().zip(alternator) {
        let Some(i) = c.to_digit(10) else { continue };
        if is_file {
            v.extend(std::iter::repeat(file_id).take(i as usize));
            file_id += 1;
        } else {
            v.extend(std::iter::repeat(-1).take(i as usize));
        }
    }

    let mut i = 0_usize;
    let mut j = v.len() - 1;
    while i < j && i < v.len() {
        if v[i] >= 0 {
            i += 1;
            continue;
        }
        if v[j] == -1 {
            j -= 1;
            continue;
        }
        (v[i], v[j]) = (v[j], v[i])
    }

    let mut sum = 0_i64;
    for (position, file_id) in v.iter().copied().enumerate() {
        if file_id < 0 {
            break;
        }
        sum += (position as i64) * (file_id as i64);
    }
    sum
}

pub fn problem_2(data: &str) -> i64 {
    let mut v: Vec<i32> = vec![];
    let mut file_id = 0;
    let alternator = Alternator { b: true };
    for (c, is_file) in data.chars().zip(alternator) {
        let Some(i) = c.to_digit(10) else { continue };
        if is_file {
            v.extend(std::iter::repeat(file_id).take(i as usize));
            file_id += 1;
        } else {
            v.extend(std::iter::repeat(-1).take(i as usize));
        }
    }

    let mut j = v.len() - 1;
    while j > 0 {
        // We find non-free space to move.
        if v[j] == -1 {
            j -= 1;
            continue;
        }
        let file_id = v[j];
        let mut j_next = j;
        while v[j_next] == file_id {
            j_next -= 1;
            if j_next == 0 {
                break;
            }
        }
        // Length of the block to move.
        let len = j - j_next;

        // elb = exclusive lower bound
        // We can start with 0 as v[0] is never free
        let mut free_space_elb = 0;
        for i in 0..j {
            if v[i] >= 0 {
                free_space_elb = i;
                continue;
            }
            let free_space = i - free_space_elb;
            if free_space < len {
                continue;
            }
            for k in (free_space_elb + 1)..=i {
                v[k] = file_id;
            }
            for k in (j_next + 1)..=j {
                v[k] = -1;
            }
            break;
        }

        j = j_next;
    }

    let mut sum = 0_i64;
    for (position, file_id) in v.iter().copied().enumerate() {
        if file_id < 0 {
            continue;
        }
        sum += (position as i64) * (file_id as i64);
    }
    sum
}

struct Alternator {
    b: bool,
}

impl Iterator for Alternator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let b = self.b;
        self.b = !self.b;
        Some(b)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "2333133121414131402";

    #[test]
    fn test_problem_1() {
        assert_eq!(1928, problem_1(DATA));
    }

    #[test]
    fn test_problem_2() {
        assert_eq!(2858, problem_2(DATA));
    }
}
