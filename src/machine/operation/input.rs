use crate::machine::{OpResult, Operation, ParamType};

pub struct Input {
    params: Vec<ParamType>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            params: vec![ParamType::Input, ParamType::Output],
        }
    }
}

impl Operation for Input {
    fn execute(&self, registry: &mut Vec<i64>) -> OpResult {
        let x = registry.get(0);
        let out_param = registry.get(1);

        if x == None || out_param == None {
            return OpResult::Error("Expected 2 Items in Stack".to_string());
        }
        let o = *out_param.unwrap() as usize;
        OpResult::Store(o, *x.unwrap())
    }

    fn params(&self) -> &[ParamType] {
        &self.params
    }
    fn name(&self) -> &'static str {
        "Input"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_missing() {
        let a = Input::new();

        let mut stack = vec![];
        stack.push(50);
        stack.push(5);

        assert_eq!(a.execute(&mut stack), OpResult::Store(5, 50))
    }
}
