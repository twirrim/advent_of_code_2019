/*

I've got a niggling feeling I really want each opcode to be its own type with various traits associated?
For example, I could have an Execute trait.

I'm really disliking all this punting about between usize and isize, but I don't think I have a choice.
day 5 input has negative numbers in it, and those numbers have to act as memory locations which are usize.
I imagine in the real world, that could be a major problem.  For AoC I imagine it's not.

*/

use std::{fmt::Display, ops::RangeInclusive};

use crate::debug_println;

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
    RelativeBaseOffset,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
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
            2 => ParameterMode::Relative,
            _ => panic!("Invalid paramter mode for position three: {third_param_value}",),
        };
        code -= third_param_value * 10000;

        let second_param_value = code / 1000;
        let second_param_mode = match second_param_value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Invalid paramter mode for position two: {second_param_value}"),
        };
        code -= second_param_value * 1000;

        let first_param_value = code / 100;
        let first_param_mode = match first_param_value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
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
            9 => OC::RelativeBaseOffset,
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
    relative_base: isize,
    input: Vec<isize>,
    output: Vec<isize>, // getting uncomfortable with this.. feels like something subject to major change later
}

impl VM {
    #[must_use]
    pub fn new(memory: Vec<isize>) -> Self {
        debug_println!("Creating VM from: {:?}", memory);
        // The computer's available memory should be much larger than the initial program.
        // Memory beyond the initial program starts with the value 0 and can be read or written like any other memory.
        // (It is invalid to try to access memory at a negative address, though.)

        // Re^:  I'm tempted to see if I can catch out-of-range memory accesses, and fill with zeroes only when necessary.
        // That pays a performance penalty on memory access, while keeping memory consumption as low as possible.
        VM {
            memory,
            pointer: 0,
            finished: false,
            relative_base: 0,
            input: vec![],
            output: vec![],
        }
    }

    pub fn run(&mut self) {
        while self.pointer < self.memory.len() && !self.finished {
            debug_println!("{:?}", self.memory);
            self.step();
        }
    }

    pub fn increment_relative_offset<T: PrimInt + Display>(&mut self, input: T) {
        debug_println!("Incrementing relative_base by {input}");
        self.relative_base += input.to_isize().unwrap();
    }

    pub fn push_input<T: PrimInt + Display>(&mut self, input: T) {
        debug_println!("Adding {input} to input queue");
        self.input.push(input.to_isize().unwrap());
    }

    pub fn pop_input(&mut self) -> isize {
        let output = self.input.pop().unwrap();
        debug_println!("Got {output} from the input queue");
        output
    }

    pub fn pop_output(&mut self) -> Option<isize> {
        let output = self.output.pop();
        debug_println!("Got {:?} from output", output);
        output
    }

    pub fn push_output(&mut self, value: isize) {
        debug_println!("Pushing {value} to output");
        self.output.push(value);
    }

    pub fn set_memory<T: PrimInt + Display>(&mut self, address: T, value: isize) {
        debug_println!("Setting {address} to {value}");
        self.memory[address.to_usize().unwrap()] = value;
    }

    pub fn get_memory<T: PrimInt + Display>(&self, address: T) -> isize {
        debug_println!("Getting from memory at {address}");
        let value = self.memory[address.to_usize().unwrap()];
        debug_println!("Got value: {value}");
        value
    }

    pub fn get_memory_range(&self, address: RangeInclusive<usize>) -> &[isize] {
        debug_println!("Getting memory range {:?}", address);
        let values = &self.memory[address];
        debug_println!("Got value: {:?}", values);
        values
    }

    fn get_param_value(&self, param_mode: &ParameterMode, location: isize) -> isize {
        // Param mode 0 is position mode, read the value to learn where to look up the final value
        // Param mode 1 is immediate mode, it's value is the final value
        match param_mode {
            ParameterMode::Position => self.get_memory(location),
            ParameterMode::Immediate => {
                debug_println!("Immediate mode, returning: {location}");
                location
            }
            ParameterMode::Relative => {
                let relative = location + self.relative_base;
                debug_println!("Relative base mode, returning {relative}");
                relative
            }
        }
    }

    fn set_pointer<T: PrimInt + Display>(&mut self, value: T) {
        debug_println!("Setting pointer to {value}");
        self.pointer = value.to_usize().unwrap();
    }

    fn increment_pointer<T: PrimInt + Display>(&mut self, value: T) {
        debug_println!("Incrementing pointer by {value}");
        self.pointer += value.to_usize().unwrap();
    }

    fn step(&mut self) {
        // From searching online, dynamic dispatch adds a bunch of undesirable overhead.
        let opcode = Opcode::new(self.get_memory(self.pointer));
        // eww opcode.opcode?
        match opcode.opcode {
            OC::Add => {
                /*
                Opcode 1 adds together numbers read from two positions and stores the
                result in a third position. The three integers immediately after the
                opcode tell you these three positions - the first two indicate the
                positions from which you should read the input values, and the third
                indicates the position at which the output should be stored.
                 */

                let parameter = self.get_memory_range(self.pointer + 1..=self.pointer + 3);
                debug_println!("{:?}: parameter: {:?}", &opcode, parameter);
                let a = self.get_param_value(&opcode.first_param_mode, parameter[0]);
                let b = self.get_param_value(&opcode.second_param_mode, parameter[1]);
                let c = parameter[2];
                debug_println!("Setting location {c} to {}", b + a);
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
                debug_println!("{:?}: parameter: {:?}", &opcode, parameter);
                let a = self.get_param_value(&opcode.first_param_mode, parameter[0]);
                let b = self.get_param_value(&opcode.second_param_mode, parameter[1]);

                let c = parameter[2];

                debug_println!("Setting location {c} to {}", b * a);
                self.set_memory(c, b * a);
                self.increment_pointer(4);
            }
            OC::End => {
                debug_println!("{:?}. Ending program", opcode.opcode);
                self.finished = true;
            }
            OC::Input => {
                /*
                Opcode 3 takes a single integer as input and saves it to the position given
                by its only parameter. For example, the instruction 3,50 would take an input
                value and store it at address 50.
                */

                let input = self.pop_input();
                let target = self.get_memory(self.pointer + 1);
                // Parameters that an instruction writes to will never be in immediate mode,
                // so no need to muck about with parameter mode
                debug_println!(
                    "{:?}, Got input {input}, writing to location {target}",
                    opcode
                );
                self.set_memory(target, input);
                self.increment_pointer(2);
            }
            OC::Output => {
                /*
                Opcode 4 outputs the value of its only parameter. For example, the
                instruction 4,50 would output the value at address 50.
                */
                let parameter = self.get_memory(self.pointer + 1);
                debug_println!("{:?}: parameter: {:?}", &opcode, parameter);
                let output = self.get_param_value(&opcode.first_param_mode, parameter);
                self.push_output(output);
                self.increment_pointer(2);
            }
            OC::JumpIfTrue => {
                /*
                Opcode 5 is jump-if-true: if the first parameter is non-zero, it
                sets the instruction pointer to the value from the second parameter.
                Otherwise, it does nothing.
                */
                let parameter = self.get_memory_range(self.pointer + 1..=self.pointer + 2);
                debug_println!("{:?}: parameter: {:?}", &opcode, parameter);
                let first_parameter_value =
                    self.get_param_value(&opcode.first_param_mode, parameter[0]);
                if first_parameter_value != 0 {
                    debug_println!("{first_parameter_value} != 0");
                    let target = self.get_param_value(&opcode.second_param_mode, parameter[1]);
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
                debug_println!("{:?}: parameter: {:?}", &opcode, parameter);

                let first_parameter_value =
                    self.get_param_value(&opcode.first_param_mode, parameter[0]);

                if first_parameter_value == 0 {
                    debug_println!("{} == 0", first_parameter_value);
                    let second_parameter_value =
                        self.get_param_value(&opcode.second_param_mode, parameter[1]);
                    self.set_pointer(second_parameter_value);
                } else {
                    debug_println!("Pointer before increment {}", self.pointer);
                    self.increment_pointer(3);
                    debug_println!("Pointer after increment {}", self.pointer);
                }
            }
            OC::LessThan => {
                /*
                Opcode 7 is less than: if the first parameter is less than the second parameter,
                it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                */

                let parameter = self.get_memory_range(self.pointer + 1..=self.pointer + 3);
                debug_println!("{:?}: parameter: {:?}", &opcode, parameter);
                let first_parameter_value =
                    self.get_param_value(&opcode.first_param_mode, parameter[0]);
                let second_parameter_value =
                    self.get_param_value(&opcode.second_param_mode, parameter[1]);
                let third_parameter_value = parameter[2];
                let store_value = if first_parameter_value < second_parameter_value {
                    debug_println!("{first_parameter_value} < {second_parameter_value}");
                    1
                } else {
                    debug_println!("{first_parameter_value} >= {second_parameter_value}");
                    0
                };

                self.set_memory(third_parameter_value, store_value);
                self.increment_pointer(4);
            }
            OC::Equals => {
                /*
                 Opcode 8 is equals: if the first parameter is equal to the second parameter,
                 it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                */
                let parameter = self.get_memory_range(self.pointer + 1..=self.pointer + 3);
                debug_println!("{:?}: parameter: {:?}", &opcode, parameter);
                let first_parameter_value =
                    self.get_param_value(&opcode.first_param_mode, parameter[0]);
                let second_parameter_value =
                    self.get_param_value(&opcode.second_param_mode, parameter[1]);
                let third_parameter_value = parameter[2];
                let store_value = if first_parameter_value == second_parameter_value {
                    debug_println!("{first_parameter_value} == {second_parameter_value}");
                    1
                } else {
                    debug_println!("{first_parameter_value} != {second_parameter_value}");
                    0
                };

                self.set_memory(third_parameter_value, store_value);
                self.increment_pointer(4);
            }
            OC::RelativeBaseOffset => {
                /*
                Opcode 9 adjusts the relative base by the value of its only parameter.
                The relative base increases (or decreases, if the value is negative) by the value of the parameter.
                 */
                let parameter = self.get_memory(self.pointer + 1);
                debug_println!("{:?}: parameter: {:?}", &opcode, parameter);
                let offset_increment = self.get_param_value(&opcode.first_param_mode, parameter);
                self.increment_relative_offset(offset_increment);
                self.increment_pointer(2);
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
    #[case(vec![5, 0, 3, 4, 5, 99], 4)] // mode 0, false
    #[case(vec![105, 1, 3, 4, 5, 99], 4)] // mode 1, true
    #[case(vec![105, 0, 6, 4, 5, 99], 3)] // mode 1, false
    fn test_op_five(#[case] input: Vec<isize>, #[case] expected: usize) {
        // five = JumpIfTrue. if first param is non-zero, should set pointer to second param
        let mut test_vm = VM::new(input);
        test_vm.step();
        assert_eq!(test_vm.pointer, expected);
    }

    #[rstest]
    #[case(vec![6, 6, 2, 4, 5, 99, 0], 2)] // mode 0, false
    #[case(vec![6, 6, 3, 4, 5, 99, 1], 3)] // mode 0, true
    #[case(vec![106, 0, 3, 4, 5, 99], 4)] // mode 1, false
    #[case(vec![106, 1, 3, 4, 5, 99], 3)] // mode 1, true
    fn test_op_six(#[case] input: Vec<isize>, #[case] expected: usize) {
        // five = JumpIfFalse. if first param is non-zero, should set pointer to second param
        let mut test_vm = VM::new(input);
        test_vm.step();
        assert_eq!(test_vm.pointer, expected);
    }

    #[rstest]
    #[case(vec![11107, 1, 2, 4, 50, 99], 1)]
    #[case(vec![7, 1, 2, 4, 4, 99], 1)]
    #[case(vec![11107, 2, 1, 4, 50, 99], 0)]
    #[case(vec![7, 2, 2, 4, 4, 99], 0)]
    fn test_op_seven(#[case] input: Vec<isize>, #[case] expected: isize) {
        // seven = LessThan. If first param less than second, store 1 in position from third
        let mut test_vm = VM::new(input);
        test_vm.step();
        assert_eq!(test_vm.memory[4], expected); // bad way to test!
    }

    #[rstest]
    #[case(vec![11108, 1, 2, 4, 50, 99], 0)]
    #[case(vec![8, 1, 2, 4, 4, 99], 0)]
    #[case(vec![11108, 2, 2, 4, 50, 99], 1)]
    #[case(vec![8, 2, 2, 4, 4, 99], 1)]
    fn test_op_eight(#[case] input: Vec<isize>, #[case] expected: isize) {
        // eight = equals. If first param = second, store 1 in position from third
        let mut test_vm = VM::new(input);
        test_vm.step();
        assert_eq!(test_vm.memory[4], expected); // bad way to test!
    }

    #[rstest]
    #[case(vec![9, 0], 9)]
    #[case(vec![109, 1], 1)]
    fn test_op_nine(#[case] input: Vec<isize>, #[case] expected: isize) {
        // nine updates relative base by provided increment
        let mut test_vm = VM::new(input);
        test_vm.step();
        assert_eq!(test_vm.relative_base, expected); // bad way to test!
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

    // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    #[rstest]
    #[case(vec![3,9,8,9,10,9,4,9,99,-1,8], 2, 0)]
    #[case(vec![3,9,8,9,10,9,4,9,99,-1,8], 8, 1)]
    #[case(vec![3,9,8,9,10,9,4,9,99,-1,8], 9, 0)]
    #[case(vec![3,3,1108,-1,8,3,4,3,99], 2, 0)]
    #[case(vec![3,3,1108,-1,8,3,4,3,99], 8, 1)]
    #[case(vec![3,3,1108,-1,8,3,4,3,99], 9, 0)]
    fn test_day5_input_is_eight(
        #[case] memory: Vec<isize>,
        #[case] input: isize,
        #[case] expected: isize,
    ) {
        let mut vm = VM::new(memory);
        vm.push_input(input);
        vm.run();
        let output = vm.pop_output();
        assert_eq!(output.unwrap(), expected);
    }

    #[rstest]
    #[case(vec![3,9,7,9,10,9,4,9,99,-1,8], 2, 1)]
    #[case(vec![3,9,7,9,10,9,4,9,99,-1,8], 8, 0)]
    #[case(vec![3,9,7,9,10,9,4,9,99,-1,8], 9, 0)]
    #[case(vec![3,3,1107,-1,8,3,4,3,99], 2, 1)]
    #[case(vec![3,3,1107,-1,8,3,4,3,99], 8, 0)]
    #[case(vec![3,3,1107,-1,8,3,4,3,99], 9, 0)]
    fn test_day5_input_less_than_eight(
        #[case] memory: Vec<isize>,
        #[case] input: isize,
        #[case] expected: isize,
    ) {
        let mut vm = VM::new(memory);
        vm.push_input(input);
        vm.run();
        let output = vm.pop_output();
        assert_eq!(output.unwrap(), expected);
    }

    #[rstest]
    #[case(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 0, 0)]
    #[case(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 0, 0)]
    #[case(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 1, 1)]
    #[case(vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], isize::MAX, 1)]
    fn test_day5_input_is_zero(
        #[case] memory: Vec<isize>,
        #[case] input: isize,
        #[case] expected: isize,
    ) {
        let mut vm = VM::new(memory);
        vm.push_input(input);
        vm.run();
        let output = vm.pop_output();
        assert_eq!(output.unwrap(), expected);
    }

    #[rstest]
    #[case(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 0, 999)]
    #[case(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 8, 1000)]
    #[case(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 9, 1001)]
    #[case(vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
        1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
        999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], isize::MAX, 1001)]
    fn test_day5_input_relates_to_eight(
        #[case] memory: Vec<isize>,
        #[case] input: isize,
        #[case] expected: isize,
    ) {
        // example program uses an input instruction to ask for a single number.
        // The program will then output 999 if the input value is below 8, output
        // 1000 if the input value is equal to 8, or output 1001 if the input value
        // is greater than 8.
        let mut vm = VM::new(memory);
        vm.push_input(input);
        vm.run();
        let output = vm.pop_output();
        assert_eq!(output.unwrap(), expected);
    }

    #[test]
    fn test_day9_example_one() {
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut vm = VM::new(input.clone());
        vm.run();
        assert_eq!(vec![1], input);
    }

    #[test]
    fn test_day9_example_two() {
        let mut vm = VM::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        vm.run();
        let output = vm.pop_output().unwrap();
        // Should be 16 digits long.. divide by 1000000000000000.
        // If it's between 1 and 9, it's a 16 digit number.
        debug_println!("{}", output / 1000000000000000);
        assert!((1..10).contains(&(output / 1000000000000000)));
    }

    #[test]
    fn test_day9_example_three() {
        let mut vm = VM::new(vec![104, 1125899906842624, 99]);
        vm.run();
        let output = vm.pop_output().unwrap();
        assert_eq!(output, 1125899906842624);
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
