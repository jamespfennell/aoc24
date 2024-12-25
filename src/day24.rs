use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

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

fn swap<'a>(
    rules: &mut HashMap<&'a str, (&str, &str, Op)>,
    swaps: &mut Vec<String>,
    a: &'a str,
    b: &'a str,
) {
    let swap_1 = *rules.get(a).unwrap();
    let swap_2 = *rules.get(b).unwrap();
    rules.insert(a, swap_2);
    rules.insert(b, swap_1);
    swaps.push(a.to_string());
    swaps.push(b.to_string());
}

pub fn problem_2(data: &str) -> String {
    let Input {
        done: _,
        pending: mut rules,
    } = Input::parse(data);
    // TODO find c00 ourselves?
    let mut mappings: HashMap<Decoded, &str> = Default::default();
    mappings.insert(Decoded('c', 0), "mcg");
    let mut swaps: Vec<String> = vec![];

    for k in 1..45 {
        let last_c = Decoded('c', k - 1);
        let last_c_code = *mappings.get(&last_c).unwrap();

        let c = Decoded('c', k);
        let f = Decoded('f', k);
        let g = Decoded('g', k);
        let h = Decoded('h', k);
        let x = Decoded('x', k);
        let y = Decoded('y', k);
        let z = Decoded('z', k);
        let x_code = x.to_string();
        let y_code = y.to_string();
        let z_code = z.to_string();
        // this makes z_code lifetime pinned to the rules
        let z_code = *rules
            .keys()
            .filter(|&&k| k == z_code.as_str())
            .next()
            .unwrap();
        // Find the C_prev XOR F rule to find F, and then ensure the rule is mapped correctly.
        for (&out, &(l, r, op)) in rules.iter() {
            let other = match (l == last_c_code, r == last_c_code) {
                (true, true) => panic!("impossible rule"),
                (true, false) => r,
                (false, true) => l,
                (false, false) => continue,
            };
            if op == Op::Xor {
                mappings.insert(f, other);
                if out != z_code {
                    swap(&mut rules, &mut swaps, out, z_code);
                }
                break;
            }
        }
        let f_code = *mappings
            .get(&f)
            .expect(&format!("failed to determine {f:?}"));

        // Ensure the X XOR Y rule is mapped correctly.
        for (&out, &(l, r, op)) in rules.iter() {
            if op == Op::Xor && ((l == x_code && r == y_code) || (l == y_code && r == x_code)) {
                if out != f_code {
                    swap(&mut rules, &mut swaps, out, f_code);
                }
                break;
            }
        }

        // find g and h
        let mut g_or = None;
        let mut h_or = None;
        for (&out, &(l, r, op)) in rules.iter() {
            if op == Op::And
                && ((l == last_c_code && r == f_code) || (l == f_code && r == last_c_code))
            {
                g_or = Some(out);
            }
            if op == Op::And && ((l == x_code && r == y_code) || (l == y_code && r == x_code)) {
                h_or = Some(out);
            }
        }
        // We must have found g and h, but note that they are only candidates!
        // We may have to swap them.
        // UPDATE: it turns out we don't.
        let g_code = g_or.unwrap();
        let h_code = h_or.unwrap();

        for (&out, &(l, r, op)) in rules.iter() {
            if op != Op::Or {
                continue;
            }
            let has_g = l == g_code || r == g_code;
            let has_h = l == h_code || r == h_code;
            match (has_h, has_g) {
                (true, true) => {
                    mappings.insert(g, g_code);
                    mappings.insert(h, h_code);
                    mappings.insert(c, out);
                }
                (true, false) | (false, true) => {
                    panic!("can't handle this case!");
                }
                // not relevant
                (false, false) => {}
            }
        }
        if !mappings.contains_key(&c) {
            panic!("failed to find {c:?}")
        }
    }
    swaps.sort();
    swaps.join(",")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Decoded(char, u8);

impl Decoded {
    fn to_string(&self) -> String {
        format!("{}{:02}", self.0, self.1)
    }
}

impl Display for Decoded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:02}", self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    );
}
