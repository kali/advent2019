use std::collections::HashMap;

fn part_1() {
    let mut machine = intcode::Machine::new_from_file("input");
    let mut h = HashMap::<(isize, isize), usize>::new();
    while !machine.done() {
        if machine.outputs.len() == 3 {
            h.insert(
                (machine.outputs[0], machine.outputs[1]),
                machine.outputs[2] as usize,
            );
            machine.outputs.clear();
        }
        machine.step();
    }
    dbg!(h.values().filter(|v| **v == 2).count());
}

fn part_2() {
    let mut machine = intcode::Machine::new_from_file("input");
    machine.mem[0] = 2;
    let mut h = HashMap::<(isize, isize), usize>::new();
    while h.is_empty() || h.values().any(|v| *v == 2) && !machine.done() {
        while !machine.waiting() && !machine.done(){
            if machine.outputs.len() == 3 {
                h.insert(
                    (machine.outputs[0], machine.outputs[1]),
                    machine.outputs[2] as usize,
                );
                machine.outputs.clear();
            }
            machine.step();
        }
        let paddle = (h.iter().find(|(_, v)| **v == 3).unwrap().0).0;
        let ball = (h.iter().find(|(_, v)| **v == 4).unwrap().0).0;
        machine
            .inputs
            .push((ball as isize - paddle as isize).signum());
    }
    dbg!(h[&(-1, 0)]);
}

fn main() {
    part_1();
    part_2();
}
