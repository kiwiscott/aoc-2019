use crate::machine::{OpResult, Operation, ParamType};

pub struct Mul {
    params: Vec<ParamType>,
}

impl Mul {
    pub fn new() -> Self {
        Self {
            params: vec![ParamType::Value, ParamType::Value, ParamType::Output],
        }
    }
}

impl Operation for Mul {
    fn execute(&self, registry: &mut Vec<i64>) -> OpResult {
        let x = registry.get(0);
        let y = registry.get(1);
        let out_param = registry.get(2);
        if x == None || y == None || out_param == None {
            return OpResult::Error("Expected 3 Items in Stack".to_string());
        }
        let o = *out_param.unwrap() as usize;

        OpResult::Store(o, x.unwrap() * y.unwrap())
    }

    fn params(&self) -> &[ParamType] {
        &self.params
    }

    fn name(&self) -> &'static str {
        "Mul"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_missing() {
        let a = Mul::new();

        let mut stack = vec![];
        stack.push(2);
        let result = a.execute(&mut stack);
        assert_eq!(
            result,
            OpResult::Error("Expected 3 Items in Stack".to_string())
        );
    }

    #[test]
    fn test_positive() {
        let a = Mul::new();

        let mut stack = vec![];
        stack.push(2);
        stack.push(22);
        stack.push(222);

        let result = a.execute(&mut stack);
        assert_eq!(result, OpResult::Store(222, 44))
    }
    #[test]
    fn test_negative() {
        let a = Mul::new();

        let mut stack = vec![];
        stack.push(2);
        stack.push(-22);
        stack.push(0);

        let result = a.execute(&mut stack);
        assert_eq!(result, OpResult::Store(0, -44))
    }
}
