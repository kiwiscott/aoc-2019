use super::*;

pub struct Machine {
    pub instructions: Vec<i64>,
    instruction_pointer: usize,
    relative_base: i64,
    registry: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
    pub state: MachineState,
}

impl Machine {
    pub fn new(instructions: Vec<i64>, input: Vec<i64>) -> Self {
        let mut buf = vec![0; 5000];
        for (i, instr) in instructions.iter().enumerate() {
            buf[i] = *instr;
        }

        Machine {
            instructions: buf,
            instruction_pointer: 0,
            relative_base: 0,
            registry: vec![],
            input: input,
            output: vec![],
            state: MachineState::New,
        }
    }
    pub fn value_at(&self, at: usize) -> i64 {
        self.instructions[at]
    }
    pub fn outputs(&self) -> Vec<i64> {
        self.output.to_vec()
    }
    pub fn insert_input(&mut self, input: i64) {
        self.input.push(input);
    }

    fn consume_pointer(&mut self) -> i64 {
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
            OpResult::Output(s) => self.output.push(s),
            OpResult::NoOp => (),
            OpResult::Jump(jump_to) => self.instruction_pointer = jump_to,
            OpResult::RelativeBase(adjust_by) => self.relative_base += adjust_by,
        }
        self.registry.clear();
    }
}
