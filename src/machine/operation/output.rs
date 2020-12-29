use crate::machine::{OpResult, Operation, ParamType};

pub struct Output {
    params: Vec<ParamType>,
}

impl Output {
    pub fn new() -> Self {
        Output {
            params: vec![ParamType::Value],
        }
    }
}

impl Operation for Output {
    fn execute(&self, registry: &mut Vec<i32>) -> OpResult {
        let x = registry.get(0);
        if x == None {
            return OpResult::Error("Expected 1 item in registry".to_string());
        }
        OpResult::Output(*x.unwrap())
    }

    fn params(&self) -> &[ParamType] {
        &self.params
    }

    fn name(&self) -> &'static str {
        "Output"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing() {
        let o = Output::new();

        let mut stack = vec![];
        stack.push(2);
        assert_eq!(OpResult::Output(2), o.execute(&mut stack));
    }
}
