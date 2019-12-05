use intcode::Machine;

fn run(program: &str, i1: isize, i2: isize) -> isize {
    let mut machine = Machine::new_from_file(program);
    machine.memory[1] = i1 as isize;
    machine.memory[2] = i2 as isize;
    let _ = machine.run(&*vec![]);
    machine.memory[0]
}

fn main() {
    dbg!(run("input", 12, 02));
    for i1 in 0..=99 {
        for i2 in 0..=99 {
            if run("input", i1, i2) == 19690720 {
                dbg!(i1 * 100 + i2);
            }
        }
    }
}
