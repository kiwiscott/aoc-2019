#[cfg(test)]
mod tests {
    use crate::machine::MachineState;
    use crate::machine::VM;

    #[test]
    fn test_part1() {
        let data = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let (mut vm, _, _) = VM::basic(&data);
        vm.process();
        assert_eq!(MachineState::Halted, vm.state());
    }

    #[test]
    fn test_output() {
        let data = [1101, 42, 10, 7, 4, 7,99,0];
        let (mut vm, _i, _o) = VM::basic(&data);
        vm.process();
        assert_eq!(Some(52), vm.last_output());
    }


    #[test]
    fn test_relative_paramas() {
        let data = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let (mut vm, _i, o) = VM::basic(&data);
        vm.process();

        assert_eq!(1219070632396864, o.recv().unwrap());
    }

    #[test]
    fn test_quine() {
        let data = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let (mut vm, _i, o) = VM::basic(&data);
        vm.process();

        for i in 0..data.len() {
            assert_eq!(data[i], o.recv().unwrap());
        }
    }

    #[test]
    fn test_large() {
        let data = [104, 1125899906842624, 99];
        let (mut vm, _i, o) = VM::basic(&data);
        vm.process();
        assert_eq!(1125899906842624, o.recv().unwrap());
    }
}
