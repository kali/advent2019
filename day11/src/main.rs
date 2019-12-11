use std::collections::HashMap;

fn run(start: bool) -> HashMap<(isize, isize), bool> {
    let mut machine = intcode::Machine::new_from_file("input");
    let mut panels = HashMap::new();
    let mut dir = 0;
    let mut position = (0isize, 0isize);
    panels.insert((0,0), start);
    loop {
        machine.inputs.push(*panels.get(&position).unwrap_or(&false) as isize);
        while machine.outputs.len() < 2 {
            if machine.done() {
                return panels;
            }
            machine.step()
        }
        panels.insert(position, machine.outputs[0] == 1);
        dir = (4 + dir + machine.outputs[1] * 2 - 1) % 4;
        let (dx, dy) = [(0, -1), (1,0), (0, 1), (-1, 0)][dir as usize];
        position = (position.0 + dx, position.1 + dy);
        machine.outputs.clear();
    }
}

fn main() {
    let part_1 = run(false);
    dbg!(part_1.len());
    let panels = run(true);
    let x_min = panels.keys().map(|k| k.0).min().unwrap();
    let x_max = panels.keys().map(|k| k.0).max().unwrap();
    let y_min = panels.keys().map(|k| k.1).min().unwrap();
    let y_max = panels.keys().map(|k| k.1).max().unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let color = panels.get(&(x,y)).copied().unwrap_or(false);
            print!("{}", if color { " "} else { "█" });
        }
        println!("");
    }
}
