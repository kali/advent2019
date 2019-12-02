use std::fs;

fn run(program: &[usize], i1: usize, i2: usize) -> usize {
    let mut memory = program.to_vec();
    memory[1] = i1;
    memory[2] = i2;
    let mut pc = 0;
    loop {
        match memory[pc] {
            99 => break,
            1 => {
                let v = memory[memory[pc + 1]] + memory[memory[pc + 2]];
                let offset = memory[pc + 3];
                memory[offset] = v;
                pc += 4;
            }
            2 => {
                let v = memory[memory[pc + 1]] * memory[memory[pc + 2]];
                let offset = memory[pc + 3];
                memory[offset] = v;
                pc += 4;
            }
            _ => panic!(),
        }
    }
    memory[0]
}

fn main() {
    let program = fs::read_to_string("input")
        .unwrap()
        .trim()
        .split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    dbg!(run(&program, 12, 02));
    for i1 in 0..=99 {
        for i2 in 0..=99 {
            if run(&program, i1, i2) == 19690720 {
                dbg!(i1*100+i2);
            }
        }
    }
}
