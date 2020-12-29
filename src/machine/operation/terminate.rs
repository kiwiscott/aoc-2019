use crate::machine::{OpResult, Operation, ParamType};

pub struct Terminate {
    params: Vec<ParamType>,
}

impl Terminate {
    pub fn new() -> Self {
        Terminate { params: vec![] }
    }
}

impl Operation for Terminate {
    fn execute(&self, _registry: &mut Vec<i32>) -> OpResult {
        OpResult::Terminate
    }

    fn params(&self) -> &[ParamType] {
        &self.params
    }

    fn name(&self) -> &'static str {
        "Terminate"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_terminate() {
        let a = Terminate::new();
        let mut stack = vec![];

        let result = a.execute(&mut stack);
        assert_eq!(result, OpResult::Terminate);
    }
}
