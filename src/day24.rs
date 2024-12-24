use std::collections::{HashMap, HashSet};

pub fn problem_1(data: &str) -> i64 {
    let Input { mut done, pending } = Input::parse(data);
    let mut edges: HashMap<&str, Vec<&str>> = Default::default();
    for (&k, (l, r, _)) in &pending {
        edges.entry(l).or_default().push(k);
        edges.entry(r).or_default().push(k);
    }
    let sorted = topological_sort(&edges);
    for node in sorted {
        let Some((l, r, op)) = pending.get(node) else {
            continue;
        };
        let l_value = *done.get(l).unwrap();
        let r_value = *done.get(r).unwrap();
        let node_value = op.calc(l_value, r_value);
        done.insert(node, node_value);
    }
    let mut sum = 0;
    for (node, value) in done {
        if !value {
            continue;
        }
        if !node.starts_with("z") {
            continue;
        }
        let idx: u32 = node[1..].parse().unwrap();
        sum += 2_i64.pow(idx);
    }
    sum
}

pub fn problem_2(_data: &str) -> i64 {
    0
}

#[derive(Clone, Copy, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn calc(&self, l: bool, r: bool) -> bool {
        match self {
            Op::And => l && r,
            Op::Or => l || r,
            Op::Xor => l != r,
        }
    }
}

#[derive(Debug)]
struct Input<'a> {
    done: HashMap<&'a str, bool>,
    pending: HashMap<&'a str, (&'a str, &'a str, Op)>,
}

impl<'a> Input<'a> {
    fn parse(data: &'a str) -> Self {
        let mut lines = data.lines();

        let mut done: HashMap<&'a str, bool> = Default::default();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let k = &line[0..3];
            let v: bool = &line[5..] == "1";
            done.insert(k, v);
        }

        let mut pending: HashMap<&'a str, (&'a str, &'a str, Op)> = Default::default();
        while let Some(line) = lines.next() {
            // mvq XOR chf -> z05
            let mut words = line.split(' ');
            let l = words.next().unwrap();
            let op = match words.next().unwrap() {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                unknown => panic!("uknown op {unknown}"),
            };
            let r = words.next().unwrap();
            words.next().unwrap(); // ->
            let k = words.next().unwrap();
            pending.insert(k, (l, r, op));
        }

        Input { done, pending }
    }
}

fn topological_sort<'a>(edges: &HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
    let mut num_deps: HashMap<&str, usize> = Default::default();
    for (source, dests) in edges {
        num_deps.entry(source).or_default();
        for &dest in dests {
            *num_deps.entry(dest).or_default() += 1;
        }
    }
    let mut ready: HashSet<&str> = num_deps
        .iter()
        .filter(|(_, &n)| n == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut sorted = vec![];
    while let Some(next) = ready.iter().next().copied() {
        sorted.push(next);
        ready.remove(next);
        let Some(dests) = edges.get(next) else {
            continue;
        };
        for dest in dests {
            let n = num_deps.get_mut(dest).unwrap();
            *n -= 1;
            if *n == 0 {
                ready.insert(dest);
            }
        }
    }
    sorted
}
#[cfg(test)]
mod test {
    use super::*;
    const DATA_1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
    const DATA_2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    super::super::tests::tests!(
        (test_problem_1_data_1, problem_1, DATA_1, 4),
        (test_problem_1_data_2, problem_1, DATA_2, 2024),
        (test_problem_2_data_1, problem_2, DATA_1, 0),
        (test_problem_2_data_2, problem_2, DATA_2, 0),
    );
}
