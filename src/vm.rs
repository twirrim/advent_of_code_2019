/*

I've got a niggling feeling I really want each opcode to be its own type with various traits associated?
For example, I could have an Execute trait.

I'm really disliking all this punting about between usize and isize, but I don't think I have a choice.
day 5 input has negative numbers in it, and those numbers have to act as memory locations which are usize.
I imagine in the real world, that could be a major problem.  For AoC I imagine it's not.

*/

use std::ops::RangeInclusive;

use crate::{debug_println, get_user_input};

use log::info;
use num_traits::int::PrimInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opcode {
    opcode: OC,
    first_param_mode: ParameterMode,
    second_param_mode: ParameterMode,
    third_param_mode: ParameterMode,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum OC {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParameterMode {
    Position,
    Immediate,
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
        let mut code = input;

        let third_param_value = input / 10000;
        let third_param_mode = match third_param_value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Invalid paramter mode for position three: {third_param_value}",),
        };
        code -= third_param_value * 10000;

        let second_param_value = code / 1000;
        let second_param_mode = match second_param_value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Invalid paramter mode for position two: {second_param_value}"),
        };
        code -= second_param_value * 1000;

        let first_param_value = code / 100;
        let first_param_mode = match first_param_value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Invalid paramter mode for position two: {first_param_value}",),
        };
        code -= first_param_value * 100;

        let opcode = match code {
            1 => OC::Add,
            2 => OC::Mul,
            3 => OC::Input,
            4 => OC::Output,
            5 => OC::JumpIfTrue,
            6 => OC::JumpIfFalse,
            7 => OC::LessThan,
            8 => OC::Equals,
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
}

impl VM {
    #[must_use]
    pub fn new(memory: Vec<isize>) -> Self {
        debug_println!("Creating VM from: {:?}", memory);
        VM {
            memory,
            pointer: 0,
            finished: false,
        }
    }

    pub fn run(&mut self) {
        while self.pointer < self.memory.len() && !self.finished {
            self.step();
        }
    }

    pub fn set_memory<T: PrimInt>(&mut self, address: T, value: isize) {
        self.memory[address.to_usize().unwrap()] = value;
    }

    pub fn get_memory<T: PrimInt>(&self, address: T) -> isize {
        self.memory[address.to_usize().unwrap()]
    }

    pub fn get_memory_range(&self, address: RangeInclusive<usize>) -> &[isize] {
        &self.memory[address]
    }

    fn get_param_value(&self, param_mode: &ParameterMode, location: isize) -> isize {
        // Param mode 0 is position mode, read the value to learn where to look up the final value
        // Param mode 1 is immediate mode, it's value is the final value
        match param_mode {
            ParameterMode::Position => self.get_memory(location),
            ParameterMode::Immediate => location,
        }
    }

    fn set_pointer<T: PrimInt>(&mut self, value: T) {
        self.pointer = value.to_usize().unwrap();
    }

    fn increment_pointer<T: PrimInt>(&mut self, value: T) {
        self.pointer += value.to_usize().unwrap();
    }

    fn step(&mut self) {
        // From searching online, dynamic dispatch adds a bunch of undesirable overhead.
        let opcode = Opcode::new(self.get_memory(self.pointer));
        // eww opcode.opcode?
        match opcode.opcode {
            // I dislike all of this being here like this, makes it hard to read
            OC::Add => {
                /*
                Opcode 1 adds together numbers read from two positions and stores the
                result in a third position. The three integers immediately after the
                opcode tell you these three positions - the first two indicate the
                positions from which you should read the input values, and the third
                indicates the position at which the output should be stored.
                 */

                let parameter = self.get_memory_range(self.pointer + 1..=self.pointer + 3);
                println!("parameter: {:?}", parameter);
                let a = self.get_param_value(&opcode.first_param_mode, parameter[0]);
                let b = self.get_param_value(&opcode.second_param_mode, parameter[1]);
                let c = self.get_param_value(&opcode.third_param_mode, parameter[2]);
                println!("a, b, c: {a}, {b}, {c}");
                self.set_memory(c, b + a);
                self.increment_pointer(4);
            }
            OC::Mul => {
                /*
                Opcode 2 works exactly like opcode 1, except it multiplies the two
                inputs instead of adding them. Again, the three integers after the
                opcode indicate where the inputs and outputs are, not their values.
                 */
                let parameter = self.get_memory_range(self.pointer + 1..=self.pointer + 3);
                let a = self.get_param_value(&opcode.first_param_mode, parameter[0]);
                let b = self.get_param_value(&opcode.second_param_mode, parameter[1]);
                let c = self.get_param_value(&opcode.third_param_mode, parameter[2]);

                self.set_memory(c, b * a);
                self.increment_pointer(4);
            }
            OC::End => self.finished = true,
            OC::Input => {
                /*
                Opcode 3 takes a single integer as input and saves it to the position given
                by its only parameter. For example, the instruction 3,50 would take an input
                value and store it at address 50.
                */

                let input = get_user_input("Provide Input: ")
                    .trim()
                    .parse::<isize>() // I think I ought to do this in the input method
                    .unwrap(); // Doubt it ever needs to accept non-integer

                let target = self.get_memory(self.pointer + 1);
                // Parameters that an instruction writes to will never be in immediate mode,
                // so no need to muck about with parameter mode
                self.set_memory(target, input);
                self.increment_pointer(2);
            }
            OC::Output => {
                /*
                Opcode 4 outputs the value of its only parameter. For example, the
                instruction 4,50 would output the value at address 50.
                */
                let parameter = self.get_memory(self.pointer + 1);
                let output = self.get_param_value(&opcode.first_param_mode, parameter);
                self.increment_pointer(2);
                info!("Output: {output}");
            }
            OC::JumpIfTrue => {
                /*
                Opcode 5 is jump-if-true: if the first parameter is non-zero, it
                sets the instruction pointer to the value from the second parameter.
                Otherwise, it does nothing.
                */
                let parameter = self.get_memory_range(self.pointer + 1..=self.pointer + 2);
                if parameter[0] != 0 {
                    let target = self.get_param_value(&opcode.first_param_mode, parameter[1]);
                    self.set_pointer(target);
                } else {
                    self.increment_pointer(3);
                }
            }
            OC::JumpIfFalse => {
                /*
                Opcode 6 is jump-if-false: if the first parameter is zero, it sets the
                instruction pointer to the value from the second parameter. Otherwise, it does nothing.
                */

                let parameter = self.get_memory_range(self.pointer + 1..=self.pointer + 2);
                if parameter[0] == 0 {
                    let target = self.get_param_value(&opcode.first_param_mode, parameter[1]);
                    self.set_pointer(target);
                } else {
                    self.increment_pointer(3);
                }
            }
            OC::LessThan => {
                /*
                Opcode 7 is less than: if the first parameter is less than the second parameter,
                it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                */

                let parameter = self.get_memory_range(self.pointer + 1..=self.pointer + 3);
                let first_parameter_value =
                    self.get_param_value(&opcode.first_param_mode, parameter[0]);
                let second_parameter_value =
                    self.get_param_value(&opcode.second_param_mode, parameter[1]);
                let third_parameter_value =
                    self.get_param_value(&opcode.third_param_mode, parameter[2]);
                let store_value = isize::from(first_parameter_value < second_parameter_value);

                self.set_memory(third_parameter_value, store_value);
                self.increment_pointer(4);
            }
            OC::Equals => {
                /*
                 Opcode 8 is equals: if the first parameter is equal to the second parameter,
                 it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                */
                let parameter = self.get_memory_range(self.pointer + 1..=self.pointer + 3);
                let first_parameter_value =
                    self.get_param_value(&opcode.first_param_mode, parameter[0]);
                let second_parameter_value =
                    self.get_param_value(&opcode.second_param_mode, parameter[1]);
                let third_parameter_value =
                    self.get_param_value(&opcode.third_param_mode, parameter[2]);
                let store_value = isize::from(first_parameter_value == second_parameter_value);

                self.set_memory(third_parameter_value, store_value);
                self.increment_pointer(4);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

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
        // Attempts to prove op code 2 functions correctly
        // Reads 1, says grab values from index 1 & 2 (1, 2), multiply together (2) and store in 5.
        let mut test_vm = VM::new(vec![2, 1, 2, 5, 99, 0]);
        test_vm.run();
        assert_eq!(test_vm.memory[5], 2);
    }

    // rstest gets me those parametrised tests I love in pytest
    #[rstest]
    #[case(vec![5, 1, 2, 4, 5, 99], 2)] // mode 0, true
    #[case(vec![5, 0, 3, 4, 5, 99], 3)] // mode 0, false
    #[case(vec![105, 1, 3, 4, 5, 99], 3)] // mode 1, true
    #[case(vec![105, 0, 3, 4, 5, 99], 3)] // mode 1, false
    fn test_op_five(#[case] input: Vec<isize>, #[case] expected: usize) {
        // five = JumpIfTrue. if first param is non-zero, should set pointer to second param
        let mut test_vm = VM::new(input);
        test_vm.step();
        assert_eq!(test_vm.pointer, expected);
    }

    // Now some specific example programs from day 2
    #[rstest]
    #[case(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99])]
    #[case(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99])]
    #[case(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801])]
    #[case(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![30, 1, 1, 4, 2, 5, 6, 0, 99])]
    fn test_day2_examples(#[case] input: Vec<isize>, #[case] expected: Vec<isize>) {
        let mut vm = VM::new(input);
        vm.run();
        assert_eq!(vm.memory, expected);
    }

    #[test]
    fn test_opcode_creation() {
        let mut test_cases = vec![(
            1002,
            Opcode {
                opcode: OC::Mul,
                first_param_mode: ParameterMode::Position,
                second_param_mode: ParameterMode::Immediate,
                third_param_mode: ParameterMode::Position,
            },
        )];
        for test_case in test_cases.iter_mut() {
            let opcode = Opcode::new(test_case.0);
            assert_eq!(opcode, test_case.1);
        }
    }
}
