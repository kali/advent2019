fn main() {
    let mut m = intcode::Machine::new_from_file("input");
    m.run(&[1]);
    dbg!(m.outputs);
    let mut m = intcode::Machine::new_from_file("input");
    m.run(&[2]);
    dbg!(m.outputs);
}
