use super::*;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct VM {
    instructions: Vec<i64>,
    instruction_pointer: usize,
    relative_base: i64,
    registry: Vec<i64>,
    input: Receiver<i64>,
    output: Sender<i64>,
    last_output: Option<i64>,
    state: MachineState,
}

impl VM {
    pub fn new(instructions: &[i64], input: Receiver<i64>, output: Sender<i64>) -> Self {
        let mut buf = vec![0; 5000];
        for (i, instr) in instructions.iter().enumerate() {
            buf[i] = *instr;
        }

        VM {
            instructions: buf,
            instruction_pointer: 0,
            relative_base: 0,
            registry: vec![],
            input: input,
            output: output,
            last_output: None,
            state: MachineState::New,
        }
    }
    pub fn basic(instructions: &[i64]) -> (Self, Sender<i64>, Receiver<i64>) {
        let (send_to, vm_input) = channel::<i64>();
        let (vm_output, rec_from) = channel::<i64>();
        let m = Self::new(instructions, vm_input, vm_output);
        (m, send_to, rec_from)
    }
    pub fn state(&self) -> MachineState {
        self.state
    }
    pub fn last_output(&self) -> Option<i64> {
        self.last_output
    }

    pub fn instruction_at(&self, at: usize) -> Option<&i64> {
        self.instructions.get(at)
    }

    fn consume_pointer(&mut self) -> i64 {
        let x = self.instructions[self.instruction_pointer];
        self.instruction_pointer += 1;
        x
    }
    pub fn process(&mut self) {
        while self.state != MachineState::Halted {
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        if self.state != MachineState::Processing {
            self.state = MachineState::Processing;
        }

        let code = self.consume_pointer();
        let (opcode, mut pmodes) = code_and_parameter_modes(code);

        let op = op_for_code(opcode);
        //Add params
        for param in op.params() {
            match param {
                ParamType::Input => {
                    let value = self.input.recv().unwrap();
                    self.registry.push(value);
                }
                ParamType::Output => {
                    let value = self.consume_pointer();
                    //Day 9 - Relative Params for Output
                    match pmodes.pop() {
                        Some(ParameterMode::Relative) => {
                            let index = self.relative_base + value;
                            self.registry.push(index);
                        }
                        _ => self.registry.push(value),
                    }
                }
                ParamType::Value => {
                    let value = self.consume_pointer();
                    match pmodes.pop() {
                        Some(ParameterMode::Relative) => {
                            let index = self.relative_base + value;
                            self.registry.push(self.instructions[index as usize]);
                        }
                        Some(ParameterMode::Immediate) => self.registry.push(value),
                        _ => self.registry.push(self.instructions[value as usize]),
                    }
                }
            }
        }

        match op.execute(&mut self.registry) {
            OpResult::Terminate => self.state = MachineState::Halted,
            OpResult::Error(s) => panic!(s),
            OpResult::Store(at, value) => self.instructions[at as usize] = value,
            OpResult::Output(s) => {
                self.last_output = Some(s);
                self.output.send(s).unwrap();
            }
            OpResult::NoOp => (),
            OpResult::Jump(jump_to) => self.instruction_pointer = jump_to,
            OpResult::RelativeBase(adjust_by) => self.relative_base += adjust_by,
        }
        self.registry.clear();
    }
}
