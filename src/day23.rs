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

pub fn problem_2(data: &str) -> String {
    // less than 3380 edges (or 6760 bidirectional)
    // for a party of size n, there are (n choose 2) = n (n-1)/2 edges (or n(n-1) bidirectional )
    // biggest party has 82 nodes

    let links = build_links(data);
    let mut max = 0;
    let mut max_nodes = vec![];
    for (&node, connected) in &links {
        let mut candidates: Vec<&str> = connected.into_iter().copied().collect();
        candidates.push(node);
        candidates.sort();
        let edges: Vec<Vec<bool>> = candidates
            .iter()
            .map(|node| {
                let edges = links.get(node).unwrap();
                candidates
                    .iter()
                    .map(|other| edges.contains(other))
                    .collect()
            })
            .collect();
        for subset in Subsets::new(candidates.len()) {
            let mut num_edges = 0;
            let mut num_nodes = 0;
            for (i, included) in subset[..candidates.len()].iter().enumerate() {
                if !included {
                    continue;
                }
                num_nodes += 1;
                for (j, included) in subset[..candidates.len()].iter().enumerate() {
                    if !included {
                        continue;
                    }
                    if edges[i][j] {
                        num_edges += 1;
                    }
                }
            }
            if (num_nodes - 1) * num_nodes == num_edges {
                if num_nodes > max {
                    max_nodes = candidates
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| subset[*i])
                        .map(|(_, node)| node)
                        .copied()
                        .collect();
                    max = num_nodes;
                }
            }
        }
    }
    max_nodes.sort();
    max_nodes.join(",")
}

struct Subsets {
    x: usize,
    max: usize,
}

impl Subsets {
    fn new(d: usize) -> Self {
        Self {
            x: 0,
            max: 2_usize.pow(d.try_into().unwrap()),
        }
    }
}

impl Iterator for Subsets {
    type Item = [bool; 16];

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.max {
            return None;
        }
        let x = self.x;
        self.x += 1;
        let mut r = [false; 16];
        (0..16).for_each(|n| {
            r[n] = (x >> n) & 1 == 0;
        });
        Some(r)
    }
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
        (
            test_problem_2_data_1,
            problem_2,
            DATA,
            "co,de,ka,ta".to_string()
        ),
    );
}
