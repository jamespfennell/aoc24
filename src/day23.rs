use std::collections::{HashMap, HashSet};

pub fn problem_1(data: &str) -> i64 {
    let links = build_links(data);
    let mut sum = 0;
    for (&first, first_out) in &links {
        if !first.starts_with("t") {
            continue;
        }
        for second in first_out {
            for third in first_out {
                if links.get(second).unwrap().contains(third) {
                    sum += match (second.starts_with("t"), third.starts_with("t")) {
                        (true, true) => 2,   // we'll see this one 6 times
                        (true, false) => 3,  // we'll see this one 4 times
                        (false, true) => 3,  // we'll see this one 4 times
                        (false, false) => 6, // 2 times
                    };
                }
            }
        }
    }
    sum / 12
}

pub fn problem_2(_data: &str) -> i64 {
    0
}

fn build_links(data: &str) -> HashMap<&str, HashSet<&str>> {
    let mut m: HashMap<&str, HashSet<&str>> = Default::default();
    for line in data.lines() {
        let mut i = line.split('-');
        let first = i.next().unwrap();
        let second = i.next().unwrap();
        m.entry(first).or_default().insert(second);
        m.entry(second).or_default().insert(first);
    }
    m
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1, DATA, 7),
        (test_problem_2_data_1, problem_2, DATA, 0),
    );
}
