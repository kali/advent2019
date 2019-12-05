use intcode::Machine;

fn main() {
    dbg!(Machine::new_from_file("input").run(&[1]));
    dbg!(Machine::new_from_file("input").run(&[5]));
}
