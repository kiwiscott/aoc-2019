use crate::machine::{OpResult, Operation, ParamType};

pub struct JumpIfTrue {
    params: Vec<ParamType>,
}

impl JumpIfTrue {
    pub fn new() -> Self {
        JumpIfTrue {
            params: vec![ParamType::Value, ParamType::Value],
        }
    }
}

impl Operation for JumpIfTrue {
    fn execute(&self, registry: &mut Vec<i64>) -> OpResult {
        let x = registry.get(0);
        let out_param = registry.get(1);

        if x == None || out_param == None {
            return OpResult::Error("Expected 2 Items in Stack".to_string());
        }
        if x.unwrap() == &0 {
            return OpResult::NoOp;
        } else {
            let o = *out_param.unwrap() as usize;
            return OpResult::Jump(o);
        }
    }
    fn params(&self) -> &[ParamType] {
        &self.params
    }
    fn name(&self) -> &'static str {
        "jump-if-true"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_missing() {
        let a = JumpIfTrue::new();

        let mut stack = vec![];
        stack.push(2);
        let result = a.execute(&mut stack);
        assert_eq!(
            result,
            OpResult::Error("Expected 2 Items in Stack".to_string())
        );
    }
    #[test]
    fn test_false() {
        let a = JumpIfTrue::new();

        let mut stack = vec![];
        stack.push(0);
        stack.push(2);
        assert_eq!(a.execute(&mut stack), OpResult::NoOp);
    }

    #[test]
    fn test_true() {
        let a = JumpIfTrue::new();

        let mut stack = vec![];
        stack.push(1);
        stack.push(4);
        assert_eq!(a.execute(&mut stack), OpResult::Jump(4));
    }
}
