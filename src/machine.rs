pub struct Machine {
    instructions: Vec<i32>,
}
impl Machine {
    pub fn new(instructions: &[i32]) -> Self {
        let v = instructions.to_vec();
        Machine { instructions: v }
    }
    pub fn value_at(&self, at: usize) -> i32 {
        self.instructions[at]
    }
    pub fn process(&mut self) {
        let mut next = 0;
        loop {
            match self.instructions.get(next) {
                Some(1) => {
                    let x = self.instructions[next + 1] as usize;
                    let y = self.instructions[next + 2] as usize;
                    let store = self.instructions[next + 3] as usize;
                    self.instructions[store] = self.instructions[x] + self.instructions[y];
                }
                Some(2) => {
                    let x = self.instructions[next + 1] as usize;
                    let y = self.instructions[next + 2] as usize;
                    let store = self.instructions[next + 3] as usize;
                    self.instructions[store] = self.instructions[x] * self.instructions[y];
                }
                Some(99) => {
                    break;
                }
                _ => {
                    panic!("Unexpected input to the  machine")
                }
            }
            next += 4;
        }
    }
}
