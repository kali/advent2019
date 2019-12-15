use std::collections::{HashMap, VecDeque};

fn main() {
    let machine = intcode::Machine::new_from_file("input");
    let mut queue = VecDeque::new();
    let mut reached = HashMap::new();
    queue.push_back((0, 0, machine));
    reached.insert((0, 0), 0);
    let machine_at_goal = 'l: loop {
        let (x, y, machine) = queue.pop_front().unwrap();
        let cost = reached[&(x,y)] + 1;
        for &dir in &[1usize, 2, 3, 4] {
            let mut machine = machine.clone();
            machine.inputs.push(dir as isize);
            while !machine.waiting() {
                machine.step();
            }
            let output = machine.outputs.remove(0);
            if output == 2 {
                dbg!(cost);
                break 'l machine;
            } else if output == 1 {
                let y2 = y + [ 0, -1, 1, 0, 0][dir];
                let x2 = x + [ 0, 0, 0, -1, 1][dir];
                if !reached.contains_key(&(x2, y2)) {
                    reached.insert((x2, y2), cost);
                    queue.push_back((x2, y2, machine));
                }
            }
        }
    };

    let mut queue = VecDeque::new();
    let mut reached = HashMap::new();
    queue.push_back((0, 0, machine_at_goal));
    reached.insert((0, 0), 0);
    while let Some((x, y, machine)) = queue.pop_front() {
        let cost = reached[&(x,y)] + 1;
        for &dir in &[1usize, 2, 3, 4] {
            let mut machine = machine.clone();
            machine.inputs.push(dir as isize);
            while !machine.waiting() {
                machine.step();
            }
            let output = machine.outputs.remove(0);
            if output != 0 {
                let y2 = y + [ 0, -1, 1, 0, 0][dir];
                let x2 = x + [ 0, 0, 0, -1, 1][dir];
                if !reached.contains_key(&(x2, y2)) {
                    reached.insert((x2, y2), cost);
                    queue.push_back((x2, y2, machine));
                }
            }
        }
    };

    dbg!(reached.values().max().unwrap());
}
