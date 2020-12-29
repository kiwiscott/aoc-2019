mod machine;
mod operation;

pub use machine::Machine;
use operation::*;

#[cfg(test)]
#[path = "machine_test.rs"]
mod machine_test;

pub(super) trait Operation {
    fn execute(&self, registry: &mut Vec<i32>) -> OpResult;
    fn params(&self) -> &[ParamType];
    fn name(&self) -> &'static str;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum ParamType {
    Input,
    Value,
    Output,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum ParameterMode {
    Immediate,
    Position,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MachineState {
    Halted,
    New,
    Processing,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum OpResult {
    Terminate,
    Error(String),
    Output(i32),
    Store(usize, i32),
    NoOp,
    Jump(usize),
}

fn op_for_code(opcode: i32) -> Box<dyn Operation> {
    match opcode {
        1 => Box::new(Add::new()),
        2 => Box::new(Mul::new()),
        3 => Box::new(Input::new()),
        4 => Box::new(Output::new()),

        5 => Box::new(JumpIfTrue::new()),
        6 => Box::new(JumpIfFalse::new()),
        7 => Box::new(LessThan::new()),
        8 => Box::new(Equals::new()),

        /*
        Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction
        pointer to the value from the second parameter. Otherwise, it does nothing.

        Opcode 6 is jump-if-false: if the first parameter is zero, it sets the
        instruction pointer to the value from the second parameter. Otherwise, it does nothing.

        Opcode 7 is less than: if the first parameter is less than the second parameter,
        it stores 1 in the position given by the third parameter. Otherwise, it stores 0.

        Opcode 8 is equals: if the first parameter is equal to the second parameter,
        it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
        */
        99 => Box::new(Terminate::new()),
        _ => panic!("Unknown Op Code: {:?}", opcode),
    }
}
fn code_and_parameter_modes(code: i32) -> (i32, Vec<ParameterMode>) {
    let mut code_string = code.to_string();
    let mut op_string = String::new();

    op_string.push(code_string.pop().unwrap());
    if let Some(v) = code_string.pop() {
        op_string = String::from(v) + &op_string;
    }

    let mut pmodes = vec![];

    loop {
        match code_string.pop() {
            Some('1') => pmodes.insert(0, ParameterMode::Immediate),
            Some('0') => pmodes.insert(0, ParameterMode::Position),
            _ => break,
        }
    }
    (op_string.parse().unwrap(), pmodes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_code_and_parameter_modes() {
        let (code, pmodes) = code_and_parameter_modes(2);
        assert_eq!(code, 2);
        assert_eq!(pmodes, []);

        let (code, pmodes) = code_and_parameter_modes(11199);
        assert_eq!(code, 99);
        assert_eq!(
            pmodes,
            [
                ParameterMode::Immediate,
                ParameterMode::Immediate,
                ParameterMode::Immediate
            ]
        );

        let (code, mut pmodes) = code_and_parameter_modes(11002);
        assert_eq!(code, 2);
        assert_eq!(Some(ParameterMode::Position), pmodes.pop());
        assert_eq!(Some(ParameterMode::Immediate), pmodes.pop());
        assert_eq!(Some(ParameterMode::Immediate), pmodes.pop());
        assert_eq!(None, pmodes.pop());
    }
}
