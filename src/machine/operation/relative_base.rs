use crate::machine::{OpResult, Operation, ParamType};

pub struct RelativeBase {
    params: Vec<ParamType>,
}

impl RelativeBase {
    pub fn new() -> Self {
        RelativeBase {
            params: vec![ParamType::Value],
        }
    }
}

impl Operation for RelativeBase {
    fn execute(&self, registry: &mut Vec<i64>) -> OpResult {
        let x = registry.get(0);
        if x == None {
            return OpResult::Error("Expected an item in Stack".to_string());
        }

        return OpResult::RelativeBase(*x.unwrap());
    }
    fn params(&self) -> &[ParamType] {
        &self.params
    }
    fn name(&self) -> &'static str {
        "relatove_base"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_missing() {
        let a = RelativeBase::new();

        let mut stack = vec![];
        let result = a.execute(&mut stack);
        assert_eq!(
            result,
            OpResult::Error("Expected an item in Stack".to_string())
        );
    }
    #[test]
    fn test_positive() {
        let a = RelativeBase::new();

        let mut stack = vec![];
        stack.push(10);
        assert_eq!(a.execute(&mut stack), OpResult::RelativeBase(10));
    }

    #[test]
    fn test_negative() {
        let a = RelativeBase::new();

        let mut stack = vec![];
        stack.push(-2);
        assert_eq!(a.execute(&mut stack), OpResult::RelativeBase(-2));
    }
}
