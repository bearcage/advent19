mod input;

#[derive(PartialEq)]
enum ReturnCode {
    Continue,
    Complete,
    Error,
}

#[derive(Debug)]
struct Interpreter {
    pub pc: usize,
    pub state: Vec<usize>,
}
impl Interpreter {
    pub fn tick(&mut self) -> ReturnCode {
        debug_assert!(self.pc <= self.state.len());
        print!(
            "@{}/{}  \t{}\t",
            self.pc,
            self.state.len(),
            self.state[self.pc]
        );
        if self.state[self.pc] == 99 {
            println!("HALT");
            return ReturnCode::Complete;
        }
        debug_assert!(self.pc + 4 <= self.state.len());
        match self.state[self.pc..self.pc + 4] {
            [1, a_ptr, b_ptr, out_ptr] => {
                let a = self.state[a_ptr];
                let b = self.state[b_ptr];
                println!("{} + {} -> {}", a, b, out_ptr);
                self.state[out_ptr] = a + b;
            }
            [2, a_ptr, b_ptr, out_ptr] => {
                let a = self.state[a_ptr];
                let b = self.state[b_ptr];
                println!("{} + {} -> {}", a, b, out_ptr);
                self.state[out_ptr] = a * b;
            }
            _ => return ReturnCode::Error,
        }
        self.pc += 4;
        ReturnCode::Continue
    }
    pub fn run(&mut self) -> ReturnCode {
        loop {
            let rc = self.tick();
            if rc != ReturnCode::Continue {
                return rc;
            }
        }
    }
}

fn solution1() {
    let mut interp = Interpreter {
        pc: 0,
        state: input::state(),
    };

    // Patch input per the problem description
    interp.state[1] = 12;
    interp.state[2] = 2;

    interp.run();
    println!("computed: {}", interp.state[0]);
}

fn gen_interp(noun: usize, verb: usize) -> Interpreter {
    let mut interp = Interpreter {
        pc: 0,
        state: input::state(),
    };

    // Patch input per the problem description
    interp.state[1] = noun;
    interp.state[2] = verb;

    interp
}

fn main() {
    let mut candidates = Vec::new();
    for i in 0..100 {
        for j in 0..100 {
            candidates.push((i, j));
        }
    }

    for (noun, verb) in candidates {
        let mut interp = gen_interp(noun, verb);
        if interp.run() == ReturnCode::Complete {
            if interp.state[0] == 19690720 {
                println!("found successful candidates: {}", 100 * noun + verb);
                return;
            }
        }
    }
    println!("found no matches :-(");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let mut interp = Interpreter {
            pc: 0,
            state: vec![1, 0, 0, 0, 99],
        };
        interp.run();
        assert_eq!(interp.state, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn example_b() {
        let mut interp = Interpreter {
            pc: 0,
            state: vec![2, 3, 0, 3, 99],
        };
        interp.run();
        assert_eq!(interp.state, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn example_c() {
        let mut interp = Interpreter {
            pc: 0,
            state: vec![2, 4, 4, 5, 99, 0],
        };
        interp.run();
        assert_eq!(interp.state, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn example_d() {
        let mut interp = Interpreter {
            pc: 0,
            state: vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
        };
        interp.run();
        assert_eq!(interp.state, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
