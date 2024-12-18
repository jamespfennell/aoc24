pub fn problem_1(data: &str) -> i64 {
    let mut machine = parse_input(data);
    let mut output: Vec<u8> = vec![];
    machine.run(|d| {
        output.push(d);
        true
    });
    let mut r = 0_i64;
    for d in &output {
        r = r * 10 + (*d as i64);
    }
    let output: Vec<String> = output.into_iter().map(|i| format!("{i}")).collect();
    println!("{}", output.join(","));
    r
}

pub fn problem_2(data: &str) -> i64 {
    let mut machine = parse_input(data);
    let raw_ins = machine.raw_ins.clone();

    // Initially tried to brute force it, but it was too slow.
    //
    // The following search algorithm is based on the observation that the program:
    // 1. is a loop
    // 2. at the start of each iteration sets b and c to some fixed function of a
    // 3. outputs one digit each iteration
    // 4. thus because of 2 and 3, the single digit output is a pure function of a
    // 5. after each iteration runs a=a/8 and exits if a==0
    //
    // So if you start with a=1000 say, then
    // - Will output digit based on a = 1000, set a = 1000/8 = 125
    // - Will output digit based on a = 125, set a = 125/8 = 15
    // - Will output digit based on a = 15, set a = 15/8 = 1
    // - Will output digit based on a = 1, then exit
    //
    // Thinking in octal, the observation is that the last digit output is just based
    // on the last octal digit of A; the second last digit output is based on the second last
    // octal digit of A and so on.
    //
    // Thus in the search algorithm we find the octal digits of A, starting from
    // the least significant.

    // Candidate values of A we are considering at each step
    let mut candidates: Vec<i64> = vec![0];
    // num_out is the number of octal digits we expect the program to output
    // for each of the candidates.
    for num_out in 1..=machine.raw_ins.len() {
        let mut new_candidates: Vec<i64> = vec![];
        for candidate in &candidates {
            for lsd in 0..8 {
                let a = candidate * 8 + lsd;
                if a == 0 {
                    continue;
                }
                machine.reg = Registers { a, b: 0, c: 0 };
                // We want to to check that the first value outputed by the program matches
                // the digit here.
                let digit_to_match = raw_ins[raw_ins.len() - num_out];
                let mut matches = false;
                machine.run(|d| {
                    matches = d == digit_to_match;
                    // We can exit early.
                    // From the previous iteration of the candidates
                    // we know that all of the other digits will match.
                    false
                });
                if matches {
                    new_candidates.push(a);
                }
            }
        }
        candidates = new_candidates;
    }
    candidates.into_iter().min().unwrap()
}

#[derive(Debug, Clone)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

#[derive(Debug, Clone)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
struct Machine {
    reg: Registers,
    ins: Vec<(Op, u8)>,
    raw_ins: Vec<u8>,
}

impl Machine {
    fn run<F: FnMut(u8) -> bool>(&mut self, mut output_fn: F) {
        let mut ip = 0_usize;
        loop {
            assert_eq!(ip % 2, 0);
            let Some((op, literal_operand)) = self.ins.get(ip / 2).cloned() else {
                break;
            };
            let combo_operand = || match literal_operand {
                0..=3 => literal_operand as i64,
                4 => self.reg.a,
                5 => self.reg.b,
                6 => self.reg.c,
                _ => panic!("unexpected literal_operand={literal_operand}"),
            };
            match op {
                Op::Adv => {
                    self.reg.a = calculate_dv(self.reg.a, combo_operand());
                }
                Op::Bxl => {
                    self.reg.b = self.reg.b ^ (literal_operand as i64);
                }
                Op::Bst => {
                    self.reg.b = combo_operand() % 8;
                }
                Op::Jnz => {
                    if self.reg.a != 0 {
                        ip = literal_operand.into();
                        continue;
                    }
                }
                Op::Bxc => {
                    self.reg.b = self.reg.b ^ self.reg.c;
                }
                Op::Out => {
                    let keep_going = output_fn((combo_operand() % 8).try_into().unwrap());
                    if !keep_going {
                        return;
                    }
                }
                Op::Bdv => {
                    self.reg.b = calculate_dv(self.reg.a, combo_operand());
                }
                Op::Cdv => {
                    self.reg.c = calculate_dv(self.reg.a, combo_operand());
                }
            }
            ip += 2;
        }
    }
}

fn calculate_dv(mut n: i64, combo_operator: i64) -> i64 {
    for _ in 0..combo_operator {
        n /= 2;
        if n == 0 {
            break;
        }
    }
    n
}
fn parse_input(data: &str) -> Machine {
    let mut lines = data.lines();
    let mut input = Machine {
        reg: Registers { a: 0, b: 0, c: 0 },
        ins: vec![],
        raw_ins: vec![],
    };
    for (prefix, target) in [
        ("Register A: ", &mut input.reg.a),
        ("Register B: ", &mut input.reg.b),
        ("Register C: ", &mut input.reg.c),
    ] {
        *target = lines
            .next()
            .unwrap()
            .strip_prefix(prefix)
            .unwrap()
            .parse()
            .unwrap();
    }
    // Empty line
    lines.next().unwrap();
    let mut iter = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .chars();
    while let Some(cl) = iter.next() {
        use Op::*;
        let op = match cl {
            '0' => Adv,
            '1' => Bxl,
            '2' => Bst,
            '3' => Jnz,
            '4' => Bxc,
            '5' => Out,
            '6' => Bdv,
            '7' => Cdv,
            _ => break,
        };
        assert_eq!(iter.next(), Some(','));
        let operand: u8 = iter
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap()
            .try_into()
            .unwrap();
        // Consume semicolon
        iter.next();
        input.ins.push((op, operand));
        input
            .raw_ins
            .push(cl.to_digit(10).unwrap().try_into().unwrap());
        input.raw_ins.push(operand);
    }
    input
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA_1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const DATA_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    super::super::tests::tests!(
        (
            test_problem_1_data_1,
            problem_1,
            DATA_1,
            4_6_3_5_6_3_5_2_1_0
        ),
        (test_problem_2_data_2, problem_2, DATA_2, 117440),
    );
}
