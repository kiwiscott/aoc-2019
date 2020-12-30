use crate::machine::{OpResult, Operation, ParamType};

pub struct LessThan {
    params: Vec<ParamType>,
}

impl LessThan {
    pub fn new() -> Self {
        LessThan {
            params: vec![ParamType::Value, ParamType::Value, ParamType::Output],
        }
    }
}

impl Operation for LessThan {
    fn execute(&self, registry: &mut Vec<i64>) -> OpResult {
        let x = registry.get(0);
        let y = registry.get(1);
        let out_param = registry.get(2);
        if x == None || y == None || out_param == None {
            return OpResult::Error("Expected 3 Items in Stack".to_string());
        }
        let o = *out_param.unwrap() as usize;

        let value = if x.unwrap() < y.unwrap() { 1 } else { 0 };
        OpResult::Store(o, value)
    }
    fn params(&self) -> &[ParamType] {
        &self.params
    }
    fn name(&self) -> &'static str {
        "less-than"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_missing() {
        let a = LessThan::new();

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
        let a = LessThan::new();

        let mut stack = vec![];
        stack.push(3);
        stack.push(33);
        stack.push(40);
        let result = a.execute(&mut stack);
        assert_eq!(result, OpResult::Store(40, 1))
    }
    #[test]
    fn test_add_negative() {
        let a = LessThan::new();

        let mut stack = vec![];
        stack.push(22);
        stack.push(2);
        stack.push(100);

        let result = a.execute(&mut stack);
        assert_eq!(result, OpResult::Store(100, 0))
    }
}
