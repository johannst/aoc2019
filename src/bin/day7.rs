type Addr = u32;
type Value = i32;

#[derive(PartialEq, Debug)]
enum StopReason {
    NeedInput,
    ProgramHalt,
}

struct IntcodeISS {
    mem: Vec<Value>,
    pc: Addr,
}

enum Instruction {
    Add(Addr, Value, Value),
    Mul(Addr, Value, Value),
    Get(Addr),
    Put(Value),
    Jpt(Value, Addr),
    Jpf(Value, Addr),
    Lt(Addr, Value, Value),
    Eq(Addr, Value, Value),
    Halt,
}

impl IntcodeISS {
    fn new(mem: &Vec<Value>) -> IntcodeISS {
        IntcodeISS {
            mem: mem.to_owned(),
            pc: 0,
        }
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

        let r1 = || self.peek(self.pc + 1);
        let r2 = || self.peek(self.pc + 2);
        let rd = || self.peek(self.pc + 3);
        let fetch = |addressing_mode, val| match addressing_mode {
            0 => self.peek(val as Addr),
            1 => val,
            _ => unimplemented!(),
        };

        match opcode {
            1 => Instruction::Add(rd() as Addr, fetch(m1, r1()), fetch(m2, r2())),
            2 => Instruction::Mul(rd() as Addr, fetch(m1, r1()), fetch(m2, r2())),
            3 => Instruction::Get(r1() as Addr),
            4 => Instruction::Put(fetch(m1, r1())),
            5 => Instruction::Jpt(fetch(m1, r1()), fetch(m2, r2()) as Addr),
            6 => Instruction::Jpf(fetch(m1, r1()), fetch(m2, r2()) as Addr),
            7 => Instruction::Lt(rd() as Addr, fetch(m1, r1()), fetch(m2, r2())),
            8 => Instruction::Eq(rd() as Addr, fetch(m1, r1()), fetch(m2, r2())),
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
    std::fs::read_to_string("input/day7").and_then(|input| {
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

fn eval_amp_chain(amp_sw: &Vec<Value>, phase_setting: [i32; 5]) -> i32 {
    let mut input = [0, 0];
    for i in 0..5 {
        input[0] = phase_setting[i]; // prepare phase setting
        let (_, output) = IntcodeISS::new(&amp_sw).compute(input.iter());
        input[1] = output[0];
    }
    input[1]
}

fn eval_amp_chain_loopback(amp_sw: &Vec<Value>, phase_setting: [i32; 5]) -> i32 {
    let mut amp_chain = Vec::new();
    let init_result = {
        let mut init_input = vec![0, 0];
        for i in 0..5 {
            init_input[0] = phase_setting[i];
            let mut iss = IntcodeISS::new(&amp_sw);
            let (_, output) = iss.compute(init_input.iter());
            init_input[1] = output[0];
            amp_chain.push(iss);
        }
        init_input[1]
    };

    let mut input = vec![init_result];
    let res = loop {
        let mut stop_reason = StopReason::ProgramHalt;

        for i in 0..5 {
            let (reason, output) = amp_chain[i].compute(input.iter());
            input = output;
            stop_reason = reason;
        }

        if stop_reason == StopReason::ProgramHalt {
            break input[0];
        }
    };

    res
}

fn gen_combinations(mut input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res_vec: Vec<Vec<i32>> = Vec::new();
    let input_len = input.len();
    if input_len == 1 {
        res_vec.push(input);
        return res_vec;
    }
    for i in (0..input_len).rev() {
        input.swap(i, input_len - 1);
        let now = input.pop().expect("Must have value!");
        let combinations = gen_combinations(input.to_owned());

        for mut comb in combinations {
            comb.push(now);
            res_vec.push(comb);
        }
        input.push(now);
    }
    res_vec
}

fn part_one() -> std::io::Result<i32> {
    let prog = read_program_from_file()?;

    let max_signal = gen_combinations(vec![0, 1, 2, 3, 4])
        .iter()
        .map(|c| {
            let mut phase_setting = [0i32; 5];
            phase_setting.copy_from_slice(&c);
            phase_setting
        })
        .fold(0, |signal, setting| {
            std::cmp::max(signal, eval_amp_chain(&prog, setting))
        });
    Ok(max_signal)
}

fn part_two() -> std::io::Result<i32> {
    let prog = read_program_from_file()?;

    let max_signal = gen_combinations(vec![5, 6, 7, 8, 9])
        .iter()
        .map(|c| {
            let mut phase_setting = [0i32; 5];
            phase_setting.copy_from_slice(&c);
            phase_setting
        })
        .fold(0, |signal, setting| {
            std::cmp::max(signal, eval_amp_chain_loopback(&prog, setting))
        });
    Ok(max_signal)
}

fn main() -> std::io::Result<()> {
    println!("Part One: max signal sent to thrusters {}", part_one()?);
    println!("Part Two: max signal sent to thrusters {}", part_two()?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one().unwrap(), 359142);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two().unwrap(), 4374895);
    }

    #[test]
    fn test_combinator() {
        let input = vec![0, 1];
        assert_eq!(gen_combinations(input), vec![vec![0, 1], vec![1, 0]]);
    }

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
    fn test_example_amp1() {
        let prog = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let phase = [4, 3, 2, 1, 0];
        assert_eq!(eval_amp_chain(&prog, phase), 43210);
    }

    #[test]
    fn test_example_amp2() {
        let prog = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phase = [0, 1, 2, 3, 4];
        assert_eq!(eval_amp_chain(&prog, phase), 54321);
    }

    #[test]
    fn test_example_amp3() {
        let prog = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phase = [1, 0, 4, 3, 2];
        assert_eq!(eval_amp_chain(&prog, phase), 65210);
    }

    #[test]
    fn test_example_amp1_loopback() {
        let prog = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let phase = [9, 8, 7, 6, 5];
        assert_eq!(eval_amp_chain_loopback(&prog, phase), 139629729);
    }

    #[test]
    fn test_example_amp2_loopback() {
        let prog = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let phase = [9, 7, 8, 5, 6];
        assert_eq!(eval_amp_chain_loopback(&prog, phase), 18216);
    }
}
