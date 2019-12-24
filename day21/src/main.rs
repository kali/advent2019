
fn main() {
    let mut vm = intcode::Machine::new_from_file("input");
    // j = !(a & b & c) & d
    let inputs = r#"NOT A T
NOT T T
AND B T
AND C T
NOT T J
AND D J
WALK
"#;
    vm.run(&inputs.bytes().map(|b| b as isize).collect::<Vec<_>>());
    dbg!(vm.outputs.last().unwrap());

    let mut vm = intcode::Machine::new_from_file("input");
    // j = !(a & b & c) & d & (e | h)
    let inputs = r#"NOT A T
NOT T T
AND B T
AND C T
NOT T J
AND D J
NOT E T
NOT T T
OR H T
AND T J
RUN
"#;
    vm.run(&inputs.bytes().map(|b| b as isize).collect::<Vec<_>>());
    dbg!(vm.outputs.last().unwrap());
}
