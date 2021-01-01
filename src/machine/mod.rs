mod operation;
mod vm;

pub use vm::VM;

use operation::*;

#[cfg(test)]
#[path = "vm_test.rs"]
mod vm_test;

pub(super) trait Operation {
    fn execute(&self, registry: &mut Vec<i64>) -> OpResult;
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
    Relative,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MachineState {
    Halted,
    New,
    Processing,
    //AwaitingInput,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(super) enum OpResult {
    Terminate,
    Error(String),
    Output(i64),
    Store(usize, i64),
    NoOp,
    RelativeBase(i64),
    Jump(usize),
}

fn op_for_code(opcode: i64) -> Box<dyn Operation> {
    match opcode {
        1 => Box::new(Add::new()),
        2 => Box::new(Mul::new()),
        3 => Box::new(Input::new()),
        4 => Box::new(Output::new()),
        5 => Box::new(JumpIfTrue::new()),
        6 => Box::new(JumpIfFalse::new()),
        7 => Box::new(LessThan::new()),
        8 => Box::new(Equals::new()),
        9 => Box::new(RelativeBase::new()),
        99 => Box::new(Terminate::new()),
        _ => panic!("Unknown Op Code: {:?}", opcode),
    }
}
fn code_and_parameter_modes(code: i64) -> (i64, Vec<ParameterMode>) {
    let mut code_string = code.to_string();
    let mut op_string = String::new();

    op_string.push(code_string.pop().unwrap());
    if let Some(v) = code_string.pop() {
        op_string = String::from(v) + &op_string;
    }

    let mut pmodes = vec![];

    loop {
        match code_string.pop() {
            Some('2') => pmodes.insert(0, ParameterMode::Relative),
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
