type MemCell = u32;

struct IntcodeISS {
    mem: [MemCell; 1024],
    pc: u32,
}

impl IntcodeISS {
    fn new() -> IntcodeISS {
        IntcodeISS {
            mem: [0; 1024],
            pc: 0,
        }
    }

    fn load_program(&mut self, prog: &[MemCell]) {
        assert!(prog.len() <= 1024);
        prog.iter().enumerate().for_each(|(i, val)| {
            self.poke(i as u32, *val);
        });
    }

    fn peek(&self, i: u32) -> MemCell {
        self.mem[i as usize]
    }

    fn poke(&mut self, i: u32, val: MemCell) {
        self.mem[i as usize] = val;
    }

    fn compute(&mut self) {
        enum IssOp {
            Step(u32),
            Halt,
        }

        loop {
            let r1 = self.peek(self.pc + 1);
            let r2 = self.peek(self.pc + 2);
            let rd = self.peek(self.pc + 3);

            let iss_op = match self.peek(self.pc) {
                1 => {
                    self.poke(rd, self.peek(r1) + self.peek(r2));
                    IssOp::Step(4)
                }
                2 => {
                    self.poke(rd, self.peek(r1) * self.peek(r2));
                    IssOp::Step(4)
                }
                99 => IssOp::Halt,
                _ => {
                    unimplemented!();
                }
            };

            match iss_op {
                IssOp::Step(len) => self.pc += len,
                IssOp::Halt => break,
            }
        }
    }
}

fn read_program_from_file() -> std::io::Result<Vec<MemCell>> {
    let fname = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: d02 <input>");
        std::process::exit(1);
    });
    std::fs::read_to_string(fname).and_then(|input| {
        let prog = input
            .split(',')
            .map(|val| {
                val.trim_end_matches('\n')
                    .parse::<MemCell>()
                    .expect(&format!("Parse {} as number failed!", val))
            })
            .collect::<Vec<MemCell>>();
        Ok(prog)
    })
}

fn main() -> std::io::Result<()> {
    let prog = read_program_from_file()?;

    let eval = |noun, verb| {
        let mut iss = IntcodeISS::new();
        iss.load_program(&prog);
        iss.poke(1, noun);
        iss.poke(2, verb);
        iss.compute();
        iss.peek(0)
    };

    // --- Part One ---
    // ... before running the program, replace position 1 with the value 12 and replace position 2
    // with the value 2.
    let result = eval(12, 2);
    println!(
        "Part One: Computer says result is {} for input noun=12 verb=2",
        result
    );

    // --- Part Two ---
    let expected_result = 19690720;
    // just simply brute force expected_result
    for noun in 0..=99 {
        for verb in 0..=99 {
            if eval(noun, verb) == expected_result {
                println!(
                    "Part Two: found expected_result={} for noun={} verb={}",
                    expected_result, noun, verb
                );
                println!("          100 * noun + verb = {}", 100 * noun + verb);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn eval(p: &Vec<MemCell>, result_pos: u32) -> MemCell {
        let mut iss = IntcodeISS::new();
        iss.load_program(&p);
        iss.compute();
        iss.peek(result_pos)
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
}
