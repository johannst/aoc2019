type Addr = u32;
type Value = i32;

struct IntcodeISS<'a> {
    mem: Vec<Value>,
    pc: Addr,
    input: std::slice::Iter<'a, Value>,
    output: Vec<Value>,
}

enum Instruction {
    Add(Addr, Value, Value),
    Mul(Addr, Value, Value),
    Get(Addr),
    Put(Value),
    JumpIfTrue(Value, Addr),
    JumpIfFalse(Value, Addr),
    Lt(Addr, Value, Value),
    Eq(Addr, Value, Value),
    Halt,
}

impl<'a> IntcodeISS<'_> {
    fn new(mem: &Vec<Value>, input: std::slice::Iter<'a, Value>) -> IntcodeISS<'a> {
        IntcodeISS {
            mem: mem.to_owned(),
            pc: 0,
            input: input,
            output: Vec::new(),
        }
    }

    fn get_output(&self) -> Vec<Value> {
        self.output.to_owned()
    }

    fn peek(&self, i: Addr) -> Value {
        self.mem[i as usize]
    }

    fn poke(&mut self, i: Addr, val: Value) {
        self.mem[i as usize] = val;
    }

    fn decode(&self, addr: Addr) -> Instruction {
        let (md, m2, m1, opcode) = {
            let word = self.peek(addr);
            (
                (word / 10000) % 10,
                (word / 1000) % 10,
                (word / 100) % 10,
                word % 100,
            )
        };
        // Parameters that an instruction writes to will never be in immediate mode.
        assert_eq!(md, 0);

        let r1 = || self.peek(self.pc + 1) as Addr;
        let r2 = || self.peek(self.pc + 2) as Addr;
        let rd = || self.peek(self.pc + 3) as Addr;
        let fetch = |addressing_mode, val| match addressing_mode {
            0 => self.peek(val),
            1 => val as Value,
            _ => unimplemented!(),
        };

        match opcode {
            1 => Instruction::Add(rd(), fetch(m1, r1()), fetch(m2, r2())),
            2 => Instruction::Mul(rd(), fetch(m1, r1()), fetch(m2, r2())),
            3 => Instruction::Get(r1()),
            4 => Instruction::Put(fetch(m1, r1())),
            5 => Instruction::JumpIfTrue(fetch(m1, r1()), fetch(m2, r2()) as Addr),
            6 => Instruction::JumpIfFalse(fetch(m1, r1()), fetch(m2, r2()) as Addr),
            7 => Instruction::Lt(rd(), fetch(m1, r1()), fetch(m2, r2())),
            8 => Instruction::Eq(rd(), fetch(m1, r1()), fetch(m2, r2())),
            99 => Instruction::Halt,
            op @ _ => {
                dbg!(op);
                unimplemented!();
            }
        }
    }

    fn compute(&mut self) {
        enum IssOp {
            Step(Addr),
            Jump(Addr),
            Halt,
        }

        loop {
            let iss_op = match self.decode(self.pc) {
                Instruction::Add(d, op1, op2) => {
                    self.poke(d, op1 + op2);
                    IssOp::Step(4)
                }
                Instruction::Mul(d, op1, op2) => {
                    self.poke(d, op1 * op2);
                    IssOp::Step(4)
                }
                Instruction::Get(d) => {
                    let i = *self
                        .input
                        .next()
                        .expect("Input stream consumed, machine still hungry!");
                    self.poke(d, i);
                    IssOp::Step(2)
                }
                Instruction::Put(op1) => {
                    self.output.push(op1);
                    println!("Intcode put: {}", op1);
                    IssOp::Step(2)
                }
                Instruction::JumpIfTrue(op1, d) => {
                    if op1 != 0 {
                        IssOp::Jump(d)
                    } else {
                        IssOp::Step(3)
                    }
                }
                Instruction::JumpIfFalse(op1, d) => {
                    if op1 == 0 {
                        IssOp::Jump(d)
                    } else {
                        IssOp::Step(3)
                    }
                }
                Instruction::Lt(d, op1, op2) => {
                    self.poke(d, (op1 < op2) as Value);
                    IssOp::Step(4)
                }
                Instruction::Eq(d, op1, op2) => {
                    self.poke(d, (op1 == op2) as Value);
                    IssOp::Step(4)
                }
                Instruction::Halt => IssOp::Halt,
            };

            match iss_op {
                IssOp::Step(len) => self.pc += len,
                IssOp::Jump(addr) => self.pc = addr,
                IssOp::Halt => break,
            }
        }
    }
}

fn read_program_from_file() -> std::io::Result<Vec<Value>> {
    let fname = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: d02 <input>");
        std::process::exit(1);
    });
    std::fs::read_to_string(fname).and_then(|input| {
        let prog = input
            .split(',')
            .map(|val| {
                val.trim_end_matches('\n')
                    .parse::<Value>()
                    .expect(&format!("Parse {} as number failed!", val))
            })
            .collect::<Vec<Value>>();
        Ok(prog)
    })
}

fn main() -> std::io::Result<()> {
    let prog = read_program_from_file()?;

    // --- Part One ---
    println!("Part One:");
    let input = vec![1]; // 1 = ID for air conditioner
    let mut iss = IntcodeISS::new(&prog, input.iter());
    iss.compute();

    // --- Part Two ---
    println!("Part Two:");
    let input = vec![5]; // 5 = ID for ship's thermal radiator controller
    let mut iss = IntcodeISS::new(&prog, input.iter());
    iss.compute();

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn eval(p: &Vec<Value>, result_pos: Addr) -> Value {
        let input = vec![];
        let mut iss = IntcodeISS::new(p, input.iter());
        iss.compute();
        iss.peek(result_pos)
    }

    fn eval_with_io(p: &Vec<Value>, input: Vec<Value>) -> Vec<Value> {
        let mut iss = IntcodeISS::new(p, input.iter());
        iss.compute();
        iss.get_output()
    }

    #[test]
    fn test_example1() {
        // 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2)
        let prog = vec![1, 0, 0, 0, 99];
        assert_eq!(eval(&prog, 0), 2);
    }

    #[test]
    fn test_example2() {
        // 2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
        let prog = vec![2, 3, 0, 3, 99];
        assert_eq!(eval(&prog, 3), 6);
    }

    #[test]
    fn test_example3() {
        // 2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
        let prog = vec![2, 4, 4, 5, 99, 0];
        assert_eq!(eval(&prog, 5), 9801);
    }

    #[test]
    fn test_example4() {
        // 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
        let prog = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(eval(&prog, 0), 30);
    }

    #[test]
    fn test_addressing_mode() {
        // 3 * [4] = 3 * 33 = 99 -> store at [4]
        let prog = vec![1002, 4, 3, 4, 33];
        assert_eq!(eval(&prog, 4), 99);

        // 100 - 1 = 99 -> store at [4]
        let prog = vec![1101, 100, -1, 4, 0];
        assert_eq!(eval(&prog, 4), 99);
    }

    #[test]
    fn test_eq_with_load() {
        // Using position mode, consider whether the input
        // is equal to 8; output 1 (if it is) or 0 (if it is not).
        let prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let input = vec![8];
        assert_eq!(eval_with_io(&prog, input), vec![1]);
        let input = vec![42];
        assert_eq!(eval_with_io(&prog, input), vec![0]);
        let input = vec![-8];
        assert_eq!(eval_with_io(&prog, input), vec![0]);
    }

    #[test]
    fn test_lt_with_load() {
        // Using position mode, consider whether the input
        // is less than 8; output 1 (if it is) or 0 (if it is not).
        let prog = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let input = vec![-42];
        assert_eq!(eval_with_io(&prog, input), vec![1]);
        let input = vec![3];
        assert_eq!(eval_with_io(&prog, input), vec![1]);
        let input = vec![8];
        assert_eq!(eval_with_io(&prog, input), vec![0]);
        let input = vec![42];
        assert_eq!(eval_with_io(&prog, input), vec![0]);
    }

    #[test]
    fn test_eq_with_immediate() {
        // Using immediate mode, consider whether the input
        // is equal to 8; output 1 (if it is) or 0 (if it is not).
        let prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let input = vec![8];
        assert_eq!(eval_with_io(&prog, input), vec![1]);
        let input = vec![42];
        assert_eq!(eval_with_io(&prog, input), vec![0]);
        let input = vec![-8];
        assert_eq!(eval_with_io(&prog, input), vec![0]);
    }

    #[test]
    fn test_lt_with_immediate() {
        // Using immediate mode, consider whether the input
        // is less than 8; output 1 (if it is) or 0 (if it is not).
        let prog = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let input = vec![-42];
        assert_eq!(eval_with_io(&prog, input), vec![1]);
        let input = vec![3];
        assert_eq!(eval_with_io(&prog, input), vec![1]);
        let input = vec![8];
        assert_eq!(eval_with_io(&prog, input), vec![0]);
        let input = vec![42];
        assert_eq!(eval_with_io(&prog, input), vec![0]);
    }

    #[test]
    fn test_jump_with_load() {
        // Take an input, then output 0 if the input was
        // zero or 1 if the input was non-zero:
        let prog = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let input = vec![0];
        assert_eq!(eval_with_io(&prog, input), vec![0]);
        let input = vec![-7];
        assert_eq!(eval_with_io(&prog, input), vec![1]);
        let input = vec![42];
        assert_eq!(eval_with_io(&prog, input), vec![1]);
    }

    #[test]
    fn test_jump_with_immediate() {
        // Take an input, then output 0 if the input was
        // zero or 1 if the input was non-zero:
        let prog = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let input = vec![0];
        assert_eq!(eval_with_io(&prog, input), vec![0]);
        let input = vec![-7];
        assert_eq!(eval_with_io(&prog, input), vec![1]);
        let input = vec![42];
        assert_eq!(eval_with_io(&prog, input), vec![1]);
    }

    #[test]
    fn test_integration() {
        // The program uses an input instruction to ask for a single number.
        // i < 8 -> output 999
        // i = 8 -> output 1000
        // i > 8 -> output 1001
        let prog = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let input = vec![-42];
        assert_eq!(eval_with_io(&prog, input), vec![999]);
        let input = vec![3];
        assert_eq!(eval_with_io(&prog, input), vec![999]);
        let input = vec![8];
        assert_eq!(eval_with_io(&prog, input), vec![1000]);
        let input = vec![42];
        assert_eq!(eval_with_io(&prog, input), vec![1001]);
    }
}
