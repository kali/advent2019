use std::fs;
use intcode::Machine;

fn run(program: &[isize], i1: isize, i2: isize) -> isize {
    let mut machine = Machine::new(program.to_vec());
    machine.memory[1] = i1 as isize;
    machine.memory[2] = i2 as isize;
    let _ = machine.run(&*vec![]);
    machine.memory[0]
}

fn main() {
    let program = fs::read_to_string("input")
        .unwrap()
        .trim()
        .split(",")
        .map(|i| i.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    dbg!(run(&program, 12, 02));
    for i1 in 0..=99 {
        for i2 in 0..=99 {
            if run(&program, i1, i2) == 19690720 {
                dbg!(i1 * 100 + i2);
            }
        }
    }
}
