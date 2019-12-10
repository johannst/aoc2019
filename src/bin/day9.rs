type Addr = usize;
type Value = i64;
const PAGE_SIZE: Addr = 1024;

#[derive(PartialEq, Debug)]
enum StopReason {
    NeedInput,
    ProgramHalt,
}

struct IntcodeISS {
    mem: Vec<Value>,
    pc: Addr,
    relative_base: Value,
}

#[derive(Debug)]
enum Instruction {
    Add(Addr, Value, Value),
    Mul(Addr, Value, Value),
    Get(Addr),
    Put(Value),
    Jpt(Value, Addr),
    Jpf(Value, Addr),
    Lt(Addr, Value, Value),
    Eq(Addr, Value, Value),
    Rbo(Value),
    Halt,
}

impl IntcodeISS {
    fn new(mem: &Vec<Value>) -> IntcodeISS {
        IntcodeISS {
            mem: mem.to_owned(),
            pc: 0,
            relative_base: 0,
        }
    }

    fn resize_mem(&mut self, addr: Addr) {
        let new_size = (addr + PAGE_SIZE) / PAGE_SIZE * PAGE_SIZE;
        self.mem.resize(new_size as usize, 0);
    }

    fn peek(&mut self, addr: Addr) -> Value {
        if let Some(cell) = self.mem.get(addr as usize) {
            *cell
        } else {
            self.resize_mem(addr);
            self.mem[addr as usize]
        }
    }

    fn poke(&mut self, addr: Addr, val: Value) {
        if let Some(cell) = self.mem.get_mut(addr as usize) {
            *cell = val;
        } else {
            self.resize_mem(addr);
            self.mem[addr as usize] = val;
        }
    }

    fn addr_fetch(&mut self, am: Value, val: Value) -> Addr {
        match am {
            0 => val as Addr,
            1 => val as Addr,
            2 => (self.relative_base + val) as Addr,
            _ => unimplemented!(),
        }
    }

    fn fetch(&mut self, am: Value, val: Value) -> Value {
        match am {
            0 => self.peek(val as Addr),
            1 => val,
            2 => self.peek((self.relative_base + val) as Addr),
            _ => unimplemented!(),
        }
    }

    fn decode(&mut self, addr: Addr) -> Instruction {
        let (md, m2, m1, opcode) = {
            let word = self.peek(addr);
            (
                (word / 10000) % 10,
                (word / 1000) % 10,
                (word / 100) % 10,
                word % 100,
            )
        };

        let r1 = self.peek(self.pc + 1);
        let r2 = self.peek(self.pc + 2);
        let rd = self.peek(self.pc + 3);
        match opcode {
            1 => Instruction::Add(
                self.addr_fetch(md, rd),
                self.fetch(m1, r1),
                self.fetch(m2, r2),
            ),
            2 => Instruction::Mul(
                self.addr_fetch(md, rd),
                self.fetch(m1, r1),
                self.fetch(m2, r2),
            ),
            3 => Instruction::Get(self.addr_fetch(m1, r1)),
            4 => Instruction::Put(self.fetch(m1, r1)),
            5 => Instruction::Jpt(self.fetch(m1, r1), self.fetch(m2, r2) as Addr),
            6 => Instruction::Jpf(self.fetch(m1, r1), self.fetch(m2, r2) as Addr),
            7 => Instruction::Lt(
                self.addr_fetch(md, rd),
                self.fetch(m1, r1),
                self.fetch(m2, r2),
            ),
            8 => Instruction::Eq(
                self.addr_fetch(md, rd),
                self.fetch(m1, r1),
                self.fetch(m2, r2),
            ),
            9 => Instruction::Rbo(self.fetch(m1, r1)),
            99 => Instruction::Halt,
            op @ _ => {
                dbg!(op);
                unimplemented!();
            }
        }
    }

    fn compute(&mut self, mut input: std::slice::Iter<'_, Value>) -> (StopReason, Vec<Value>) {
        enum IssOp {
            Step(Addr),
            Jump(Addr),
            Halt,
        }

        let mut output = Vec::new();
        let reason = loop {
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
                    if let Some(&i) = input.next() {
                        self.poke(d, i);
                        IssOp::Step(2)
                    } else {
                        break StopReason::NeedInput;
                    }
                }
                Instruction::Put(op1) => {
                    output.push(op1);
                    IssOp::Step(2)
                }
                Instruction::Jpt(op1, d) => {
                    if op1 != 0 {
                        IssOp::Jump(d)
                    } else {
                        IssOp::Step(3)
                    }
                }
                Instruction::Jpf(op1, d) => {
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
                Instruction::Rbo(op1) => {
                    self.relative_base += op1;
                    IssOp::Step(2)
                }
                Instruction::Halt => IssOp::Halt,
            };

            match iss_op {
                IssOp::Step(len) => self.pc += len,
                IssOp::Jump(addr) => self.pc = addr,
                IssOp::Halt => break StopReason::ProgramHalt,
            }
        };

        (reason, output)
    }
}

fn read_program_from_file() -> std::io::Result<Vec<Value>> {
    std::fs::read_to_string("input/day9").and_then(|input| {
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

fn part_one() -> std::io::Result<Value> {
    let prog = read_program_from_file()?;
    let input = vec![1];

    let mut iss = IntcodeISS::new(&prog);
    let (_, output) = iss.compute(input.iter());
    assert_eq!(output.len(), 1);
    Ok(output[0])
}

fn part_two() -> std::io::Result<Value> {
    let prog = read_program_from_file()?;
    let input = vec![2];

    let mut iss = IntcodeISS::new(&prog);
    let (_, output) = iss.compute(input.iter());
    assert_eq!(output.len(), 1);
    Ok(output[0])
}

fn main() -> std::io::Result<()> {
    println!("Part One: BOOST keycode {}", part_one()?);
    println!(
        "Part Two: oordinates of the distress signal {}",
        part_two()?
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn eval(p: &Vec<Value>, result_pos: Addr) -> Value {
        let input = vec![];
        let mut iss = IntcodeISS::new(p);
        iss.compute(input.iter());
        iss.peek(result_pos)
    }

    fn eval_with_io(p: &Vec<Value>, input: Vec<Value>) -> Vec<Value> {
        let mut iss = IntcodeISS::new(p);
        let (reason, output) = iss.compute(input.iter());
        assert_eq!(reason, StopReason::ProgramHalt);
        output
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

    #[test]
    fn test_boost_example1() {
        let prog = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        assert_eq!(eval_with_io(&prog, vec![]), prog);
    }

    #[test]
    fn test_boost_example2() {
        let prog = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let output = eval_with_io(&prog, vec![]);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0].to_string().chars().count(), 16);
    }

    #[test]
    fn test_boost_example3() {
        let prog = vec![104, 1125899906842624, 99];
        assert_eq!(eval_with_io(&prog, vec![]), vec![1125899906842624]);
    }
}
