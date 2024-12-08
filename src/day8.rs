use std::collections::{HashMap, HashSet};


pub fn problem_1(data: &str) -> i64 {
    let num_r = data.lines().count() as isize;
    let num_c = data.lines().next().unwrap().chars().count() as isize;
    let mut frequences: HashMap<char, Vec<(usize, usize)>> = Default::default();
    for (r, line) in data.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '.' {
                continue;
            }
            frequences.entry(ch).or_default().push((r, c));
        }
    }
    let mut antinodes: HashSet<(isize, isize)> = Default::default();
    for (_, pos) in &frequences {
        for i in 0..pos.len() {
            for j in 0..pos.len() {
                if i == j {
                    continue;
                }
                let (r1, c1) = (pos[i].0 as isize, pos[i].1 as isize);
                let (r2, c2) = (pos[j].0 as isize, pos[j].1 as isize);

                let cand = (r2 + (r2 - r1), c2 + (c2 - c1));
                if cand.0 >= 0 && cand.0 < num_r && cand.1 >= 0 && cand.1 < num_c {
                    antinodes.insert(cand);
                }
            }
        }
    }
    antinodes.len().try_into().unwrap()
}

pub fn problem_2(_data: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_problem_1() {
        assert_eq!(14, problem_1(DATA));
    }

    #[test]
    fn test_problem_2() {
        assert_eq!(0, problem_2(DATA));
    }
}
