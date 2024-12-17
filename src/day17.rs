pub fn problem_1(data: &str) -> i64 {
    let mut machine = parse_input(data);
    println!("ins={:?}", machine.ins);
    let output = machine.run();
    let output: Vec<String> = output.into_iter().map(|i| format!("{i}")).collect();
    println!("{}", output.join(","));
    1
}

pub fn problem_2(_data: &str) -> i64 {
    0
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
    ip: usize,
}

impl Machine {
    fn run(&mut self) -> Vec<u8> {
        let mut output = vec![];
        loop {
            assert_eq!(self.ip % 2, 0);
            let Some((op, literal_operand)) = self.ins.get(self.ip / 2).cloned() else {
                break;
            };
            // println!("ip={}", self.ip);
            let combo_operand = || {match literal_operand {
                0..=3 => literal_operand as i64,
                4 => self.reg.a,
                5 => self.reg.b,
                6 => self.reg.c,
                _ => panic!("unexpected literal_operand={literal_operand}"),
            }};
            /*
            The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.

            The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.

            The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.

            The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.

            The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)

            The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)

            The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)

            The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
            */
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
                        self.ip = literal_operand.into();
                        continue;
                    }
                }
                Op::Bxc => {
                    self.reg.b = self.reg.b ^ self.reg.c;
                }
                Op::Out => {
                    output.push((combo_operand() % 8).try_into().unwrap());
                }
                Op::Bdv => {
                    self.reg.b = calculate_dv(self.reg.a, combo_operand());
                }
                Op::Cdv => {
                    self.reg.c = calculate_dv(self.reg.a, combo_operand());
                }
            }
            self.ip += 2;
        }
        output
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
        ip: 0,
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
    }
    input
}

#[cfg(test)]
mod test {
    use super::*;
    const DATA: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    super::super::tests::tests!(
        // 4,6,3,5,6,3,5,2,1,0
        (test_problem_1_data_1, problem_1, DATA, 0),
        (test_problem_2_data_1, problem_2, DATA, 0),
    );
}
