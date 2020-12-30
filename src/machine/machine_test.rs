use crate::machine::Machine;

#[test]
fn test_part1() {
    let data = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let mut m = Machine::new(data.to_vec(), vec![]);
    m.process();
    assert_eq!(3500, m.value_at(0));
}

#[test]
fn test_relative_paramas() {
    let data = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let mut m = Machine::new(data.to_vec(), vec![]);
    m.process();
    assert_eq!(1219070632396864, m.outputs()[0]);
}

#[test]
fn test_quine() {
    let data = [
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut m = Machine::new(data.to_vec(), vec![]);
    m.process();
    for i in 0..data.len() {
        assert_eq!(data[i], m.outputs()[i]);
    }
}

#[test]
fn test_large() {
    let data = [104, 1125899906842624, 99];
    let mut m = Machine::new(data.to_vec(), vec![]);
    m.process();
    assert_eq!(1125899906842624, m.outputs()[0]);
}
