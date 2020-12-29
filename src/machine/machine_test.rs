use crate::machine::Machine;

#[test]
fn test_part1() {
    let data = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let mut m = Machine::new(&data);
    m.process();
    assert_eq!(3500, m.value_at(0));
}
