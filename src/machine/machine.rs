use super::*;

pub struct Machine {
    pub instructions: Vec<i32>,
    instruction_pointer: usize,
    registry: Vec<i32>,
    input: Vec<i32>,
    output: Vec<i32>,
    pub state: MachineState,
}

impl Machine {
    pub fn new(instructions: Vec<i32>, input: Vec<i32>) -> Self {
        Machine {
            instructions: instructions,
            instruction_pointer: 0,
            registry: vec![],
            input: input,
            output: vec![],
            state: MachineState::New,
        }
    }
    pub fn value_at(&self, at: usize) -> i32 {
        self.instructions[at]
    }
    pub fn outputs(&self) -> Vec<i32> {
        self.output.to_vec()
    }
    pub fn insert_input(&mut self, input: i32) {
        self.input.push(input);
    }

    fn consume_pointer(&mut self) -> i32 {
        let x = self.instructions[self.instruction_pointer];
        self.instruction_pointer += 1;
        x
    }
    pub fn process(&mut self) {
        while self.state != MachineState::Halted {
            self.next();
        }
    }

    pub fn next(&mut self) {
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
                    let value = self.input.remove(0);
                    self.registry.push(value);
                }
                ParamType::Output => {
                    let value = self.consume_pointer();
                    self.registry.push(value);
                }
                ParamType::Value => {
                    let value = self.consume_pointer();
                    match pmodes.pop() {
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
            OpResult::Output(s) => self.output.push(s),
            OpResult::NoOp => (),
            OpResult::Jump(jump_to) => self.instruction_pointer = jump_to,
        }
        self.registry.clear();
    }
}
