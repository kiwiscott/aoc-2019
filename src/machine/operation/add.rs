use crate::machine::{OpResult, Operation, ParamType};

pub struct Add {
    params: Vec<ParamType>,
}

impl Add {
    pub fn new() -> Self {
        Add {
            params: vec![ParamType::Value, ParamType::Value, ParamType::Output],
        }
    }
}

impl Operation for Add {
    fn execute(&self, registry: &mut Vec<i32>) -> OpResult {
        let x = registry.get(0);
        let y = registry.get(1);
        let out_param = registry.get(2);
        if x == None || y == None || out_param == None {
            return OpResult::Error("Expected 3 Items in Stack".to_string());
        }
        let o = *out_param.unwrap() as usize;

        OpResult::Store(o, x.unwrap() + y.unwrap())
    }
    fn params(&self) -> &[ParamType] {
        &self.params
    }
    fn name(&self) -> &'static str {
        "Add"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_missing() {
        let a = Add::new();

        let mut stack = vec![];
        stack.push(2);
        let result = a.execute(&mut stack);
        assert_eq!(
            result,
            OpResult::Error("Expected 3 Items in Stack".to_string())
        );
    }

    #[test]
    fn test_add() {
        let a = Add::new();

        let mut stack = vec![];
        stack.push(3);
        stack.push(33);
        stack.push(1);
        let result = a.execute(&mut stack);
        assert_eq!(result, OpResult::Store(1, 36))
    }
    #[test]
    fn test_add_negative() {
        let a = Add::new();

        let mut stack = vec![];
        stack.push(2);
        stack.push(-22);
        stack.push(100);

        let result = a.execute(&mut stack);
        assert_eq!(result, OpResult::Store(100, -20))
    }
}
