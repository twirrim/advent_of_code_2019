use std::ops::RangeInclusive;

use log::debug;

// I've got a niggling feeling I really want each instruction to be its own type.
// with various traits associated.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opcode {
    opcode: OC,
    first_param_mode: isize,
    second_param_mode: isize,
    third_param_mode: isize,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum OC {
    Add,
    Mul,
    End,
}

impl Opcode {
    fn new(input: isize) -> Self {
        debug!("Building opcode from {}", input);
        let mut code = input.clone();
        let third_param_mode = input / 10000;
        code -= third_param_mode * 10000;
        let second_param_mode = code / 1000;
        code -= second_param_mode * 1000;
        let first_param_mode = code / 100;
        code -= first_param_mode * 100;

        let opcode = match code {
            1 => OC::Add,
            2 => OC::Mul,
            99 => OC::End,
            _ => panic!("Invalid opcode: {code}"),
        };

        Opcode {
            opcode,
            first_param_mode,
            second_param_mode,
            third_param_mode,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    opcode: Opcode,
    first: Option<isize>,
    second: Option<isize>,
    third: Option<isize>,
}

#[derive(Debug, Clone)]
pub struct VM {
    memory: Vec<isize>,
    pointer: usize,
    finished: bool,
}

impl VM {
    #[must_use]
    pub fn new(input: Vec<isize>) -> Self {
        debug!("Building VM from: {:?}", input);
        VM {
            memory: input,
            pointer: 0,
            finished: false,
        }
    }

    pub fn run(&mut self) {
        while self.pointer < self.memory.len() && !self.finished {
            self.step();
        }
    }

    pub fn set_memory(&mut self, address: usize, value: isize) {
        self.memory[address] = value;
    }

    pub fn get_memory(&mut self, address: usize) -> isize {
        self.memory[address]
    }

    pub fn get_memory_range(&mut self, address: RangeInclusive<usize>) -> &[isize] {
        &self.memory[address]
    }

    fn step(&mut self) {
        // Ideally, I think I want to look at dynamic dispatch.  This'll do for now
        match self.get_memory(self.pointer) {
            1 => {
                /*
                Opcode 1 adds together numbers read from two positions and stores the
                result in a third position. The three integers immediately after the
                opcode tell you these three positions - the first two indicate the
                positions from which you should read the input values, and the third
                indicates the position at which the output should be stored.
                 */
                let instruction = &self.get_memory_range(self.pointer + 1..=self.pointer + 3);
                let a: usize = instruction[0].try_into().unwrap();
                let b: usize = instruction[1].try_into().unwrap();
                let c: usize = instruction[2].try_into().unwrap();
                self.set_memory(c, self.memory[b] + self.memory[a]);
                self.pointer += 4;
            }
            2 => {
                /*
                Opcode 2 works exactly like opcode 1, except it multiplies the two
                inputs instead of adding them. Again, the three integers after the
                opcode indicate where the inputs and outputs are, not their values.
                 */
                let instruction = &self.get_memory_range(self.pointer + 1..=self.pointer + 3);
                let a: usize = instruction[0].try_into().unwrap();
                let b: usize = instruction[1].try_into().unwrap();
                let c: usize = instruction[2].try_into().unwrap();
                self.set_memory(c, self.memory[b] * self.memory[a]);
                self.pointer += 4;
            }
            99 => self.finished = true,
            _ => panic!("{} not implemented yet!", self.memory[self.pointer]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_nine_nine() {
        // Attempts to prove op code 99 functions correctly
        // if 99 isn't executed correctly, machine won't be in finished state, even if it stops running
        let mut test_vm = VM::new(vec![99]);
        assert_eq!(test_vm.finished, false);
        test_vm.run();
        assert_eq!(test_vm.finished, true);
    }

    #[test]
    fn test_op_one() {
        // Attempts to prove op code 1 functions correctly
        // Reads 1, says grab values from index 1 & 2 (1, 2), add together (3) and store in 5.
        let mut test_vm = VM::new(vec![1, 1, 2, 5, 99, 0]);
        test_vm.run();
        assert_eq!(test_vm.memory[5], 3);
    }

    #[test]
    fn test_op_two() {
        // Attempts to prove op code 1 functions correctly
        // Reads 1, says grab values from index 1 & 2 (1, 2), multiply together (2) and store in 5.
        let mut test_vm = VM::new(vec![2, 1, 2, 5, 99, 0]);
        test_vm.run();
        assert_eq!(test_vm.memory[5], 2);
    }

    // Now some specific example programs from day 2
    #[test]
    fn test_day2_examples() {
        let mut test_cases = vec![
            (VM::new(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]),
            (VM::new(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]),
            (VM::new(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]),
            (
                VM::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            ),
        ];
        for test_case in test_cases.iter_mut() {
            test_case.0.run();
            assert_eq!(test_case.0.memory, test_case.1);
        }
    }

    #[test]
    fn test_opcode_creation() {
        let mut test_cases = vec![(
            1002,
            Opcode {
                opcode: 2,
                first_param_mode: 0,
                second_param_mode: 1,
                third_param_mode: 0,
            },
        )];
        for test_case in test_cases.iter_mut() {
            let opcode = Opcode::new(test_case.0);
            assert_eq!(opcode, test_case.1);
        }
    }
}
