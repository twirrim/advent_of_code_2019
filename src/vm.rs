use std::ops::RangeInclusive;

// use log::debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opcode {
    opcode: OC,
    first_param_mode: isize,
    second_param_mode: isize,
    third_param_mode: isize,
}

// I've got a niggling feeling I really want each opcode to be its own type.
// with various traits associated?

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum OC {
    Add,
    Mul,
    Input,
    Output,
    End,
}

impl Opcode {
    fn new(input: isize) -> Self {
        /*
        Parameter modes are stored in the same value as the instruction's opcode.
        The opcode is a two-digit number based only on the ones and tens digit of
        the value, that is, the opcode is the rightmost two digits of the first
        value in an instruction. Parameter modes are single digits, one per
        parameter, read right-to-left from the opcode: the first parameter's mode
        is in the hundreds digit, the second parameter's mode is in the thousands digit,
        the third parameter's mode is in the ten-thousands digit,and so on.

        Any missing modes are 0.

         */
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
            3 => OC::Input,
            4 => OC::Output,
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
pub struct VM {
    memory: Vec<isize>,
    pointer: usize,
    finished: bool,
    input: Option<isize>,
}

impl VM {
    #[must_use]
    pub fn new(memory: Vec<isize>, input: Option<isize>) -> Self {
        VM {
            memory,
            pointer: 0,
            finished: false,
            input: input,
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

    pub fn get_memory(&self, address: usize) -> isize {
        self.memory[address]
    }

    pub fn get_memory_range(&self, address: RangeInclusive<usize>) -> &[isize] {
        &self.memory[address]
    }

    fn step(&mut self) {
        // Ideally, I think I want to look at dynamic dispatch.  This'll do for now
        let opcode = Opcode::new(self.get_memory(self.pointer));
        // eww opcode.opcode?
        match opcode.opcode {
            // I dislike all of this being here like this, makes it hard to read, and I really dislike the duplication around param modes.
            OC::Add => {
                /*
                Opcode 1 adds together numbers read from two positions and stores the
                result in a third position. The three integers immediately after the
                opcode tell you these three positions - the first two indicate the
                positions from which you should read the input values, and the third
                indicates the position at which the output should be stored.
                 */

                let instruction = self.get_memory_range(self.pointer + 1..=self.pointer + 3);
                // Param mode 0 is position mode, read the value to learn where to look up the final value
                // Param mode 1 is immediate mode, it's value is the final value
                let a = match opcode.first_param_mode {
                    0 => self.get_memory(instruction[0] as usize),
                    1 => instruction[0],
                    _ => panic!("Unknown first param mode: {}", opcode.first_param_mode),
                };

                let b = match opcode.second_param_mode {
                    0 => self.get_memory(instruction[1] as usize),
                    1 => instruction[1],
                    _ => panic!("Unknown second param mode: {}", opcode.second_param_mode),
                };

                let c = instruction[2];
                self.set_memory(c as usize, b + a);
                self.pointer += 4;
            }
            OC::Mul => {
                /*
                Opcode 2 works exactly like opcode 1, except it multiplies the two
                inputs instead of adding them. Again, the three integers after the
                opcode indicate where the inputs and outputs are, not their values.
                 */
                let instruction = self.get_memory_range(self.pointer + 1..=self.pointer + 3);
                let a = match opcode.first_param_mode {
                    0 => self.get_memory(instruction[0] as usize),
                    1 => instruction[0],
                    _ => panic!("Unknown first param mode: {}", opcode.first_param_mode),
                };

                let b = match opcode.second_param_mode {
                    0 => self.get_memory(instruction[1] as usize),
                    1 => instruction[1],
                    _ => panic!("Unknown second param mode: {}", opcode.second_param_mode),
                };

                let c = instruction[2];

                self.set_memory(c as usize, b * a);
                self.pointer += 4;
            }
            OC::End => self.finished = true,
            OC::Input => {
                /*
                Opcode 3 takes a single integer as input and saves it to the position given
                by its only parameter. For example, the instruction 3,50 would take an input
                value and store it at address 50.

                // NOTE: I _suspect_ from the way references to taking an input come across, it
                _likely_ means this should be interactive
                 */
                match self.input {
                    Some(x) => {
                        let target = self.get_memory(self.pointer + 1);
                        self.set_memory(target as usize, x);
                    }
                    _ => panic!("Input opcode with no input makes no sense"),
                }
                self.pointer += 2;
            }
            OC::Output => todo!("{:?}", opcode),
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
        let mut test_vm = VM::new(vec![99], None);
        assert_eq!(test_vm.finished, false);
        test_vm.run();
        assert_eq!(test_vm.finished, true);
    }

    #[test]
    fn test_op_one() {
        // Attempts to prove op code 1 functions correctly
        // Reads 1, says grab values from index 1 & 2 (1, 2), add together (3) and store in 5.
        let mut test_vm = VM::new(vec![1, 1, 2, 5, 99, 0], None);
        test_vm.run();
        assert_eq!(test_vm.memory[5], 3);
    }

    #[test]
    fn test_op_two() {
        // Attempts to prove op code 1 functions correctly
        // Reads 1, says grab values from index 1 & 2 (1, 2), multiply together (2) and store in 5.
        let mut test_vm = VM::new(vec![2, 1, 2, 5, 99, 0], None);
        test_vm.run();
        assert_eq!(test_vm.memory[5], 2);
    }

    // Now some specific example programs from day 2
    #[test]
    fn test_day2_examples() {
        let mut test_cases = vec![
            (VM::new(vec![1, 0, 0, 0, 99], None), vec![2, 0, 0, 0, 99]),
            (VM::new(vec![2, 3, 0, 3, 99], None), vec![2, 3, 0, 6, 99]),
            (
                VM::new(vec![2, 4, 4, 5, 99, 0], None),
                vec![2, 4, 4, 5, 99, 9801],
            ),
            (
                VM::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], None),
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            ),
        ];
        for test_case in test_cases.iter_mut() {
            println!("Test Case: {:?}", test_case);
            test_case.0.run();
            assert_eq!(test_case.0.memory, test_case.1);
        }
    }

    #[test]
    fn test_opcode_creation() {
        let mut test_cases = vec![(
            1002,
            Opcode {
                opcode: OC::Mul,
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
