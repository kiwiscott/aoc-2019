use super::*;

pub struct Machine {
    instructions: Vec<i32>,
    instruction_pointer: usize,
    registry: Vec<i32>,
    pub input: Box<dyn Fn() -> &'static str>,
    pub output: Box<dyn Fn(&str)>,
}
impl Machine {
    pub fn new(instructions: &[i32]) -> Self {
        let v = instructions.to_vec();
        fn swallow(_: &str) {}

        fn no_input() -> &'static str {
            panic!("No input provided")
        }

        Machine {
            instructions: v,
            instruction_pointer: 0,
            registry: vec![],
            input: Box::new(no_input),
            output: Box::new(swallow),
        }
    }
    pub fn value_at(&self, at: usize) -> i32 {
        self.instructions[at]
    }

    fn consume_pointer(&mut self) -> i32 {
        let x = self.instructions[self.instruction_pointer];
        self.instruction_pointer += 1;
        x
    }

    pub fn process(&mut self) {
        loop {
            let code = self.consume_pointer();
            let (opcode, mut pmodes) = code_and_parameter_modes(code);

            let op = op_for_code(opcode);
            //Add params
            for param in op.params() {
                match param {
                    ParamType::Input => {
                        let c = (self.input)();
                        self.registry.push(c.parse().unwrap());
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
                OpResult::Terminate => break,
                OpResult::Error(s) => panic!(s),
                OpResult::Store(at, value) => self.instructions[at as usize] = value,
                OpResult::Output(s) => (self.output)(&s.to_string()),
                OpResult::NoOp => (),
                OpResult::Jump(jump_to) => self.instruction_pointer = jump_to
            }
            self.registry.clear();
        }
    }
}
