use std::{fmt, fs, path};

#[derive(Clone)]
pub struct Machine {
    pub inputs: Vec<isize>,
    pub outputs: Vec<isize>,
    pub memory: Vec<isize>,
    pub pc: usize,
}

impl fmt::Debug for Machine {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.memory.len() {
            if i % 10 == 0 {
                write!(fmt, "\n{:04} ", i)?;
            }
            if i % 5 == 0 {
                write!(fmt, "| ")?;
            }
            if self.pc == i {
                write!(fmt, "[{:5}]", self.memory[i])?;
            } else {
                write!(fmt, " {:5} ", self.memory[i])?;
            }
        }
        write!(fmt, "\n")
    }
}

impl Machine {
    pub fn new(memory: Vec<isize>) -> Machine {
        Machine {
            memory,
            pc: 0,
            inputs: vec![],
            outputs: vec![],
        }
    }

    pub fn new_from_file<P: AsRef<path::Path>>(p: P) -> Machine {
        let program = fs::read_to_string(p)
            .unwrap()
            .trim()
            .split(",")
            .map(|i| i.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        Machine::new(program)
    }

    pub fn imm(&self, offset: usize) -> isize {
        self.memory[self.pc + offset as usize]
    }

    pub fn arg(&self, offset: usize) -> isize {
        let opcode = self.imm(0) as usize;
        if opcode / 10usize.pow(offset as u32 + 1) % 10 == 1 {
            self.imm(offset)
        } else {
            self.memory[self.imm(offset) as usize]
        }
    }

    pub fn arg_store(&mut self, offset: usize, v: isize) {
        let offset = self.imm(offset);
        self.memory[offset as usize] = v;
    }

    pub fn done(&self) -> bool {
        self.imm(0) == 99
    }

    pub fn step(&mut self) {
        let opcode = self.imm(0);
        match opcode % 100 {
            1 => {
                self.arg_store(3, self.arg(1) + self.arg(2));
                self.pc += 4;
            }
            2 => {
                self.arg_store(3, self.arg(1) * self.arg(2));
                self.pc += 4;
            }
            3 => {
                let v = self.inputs.remove(0);
                self.arg_store(1, v);
                self.pc += 2;
            }
            4 => {
                self.outputs.push(self.arg(1));
                self.pc += 2;
            }
            5 => {
                if self.arg(1) == 0 {
                    self.pc += 3;
                } else {
                    self.pc = self.arg(2) as usize;
                }
            }
            6 => {
                if self.arg(1) != 0 {
                    self.pc += 3;
                } else {
                    self.pc = self.arg(2) as usize;
                }
            }
            7 => {
                self.arg_store(3, (self.arg(1) < self.arg(2)) as usize as isize);
                self.pc += 4;
            }
            8 => {
                self.arg_store(3, (self.arg(1) == self.arg(2)) as usize as isize);
                self.pc += 4;
            }
            x => panic!("unknown instruction {}", x),
        }
    }

    pub fn run(&mut self, inputs: &[isize]) -> &[isize] {
        self.inputs.extend(inputs);
        while !self.done() {
            self.step()
        }
        &*self.outputs
    }
}

#[test]
pub fn test_day02() {
    let mut machine = Machine::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    machine.run(&[]);
    assert_eq!(machine.memory[0], 3500);
}

#[test]
pub fn test_day05_imm() {
    let mut machine = Machine::new(vec![1002, 4, 3, 4, 33]);
    machine.run(&[]);
    assert_eq!(machine.memory[4], 99);
}

#[test]
pub fn test_day05_io() {
    let mut machine = Machine::new(vec![3, 0, 4, 0, 99]);
    assert_eq!(machine.run(&[57]), &[57]);
}

#[test]
pub fn test_day05_part2() {
    for i in 0..10 {
        let mut machine = Machine::new(vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]);
        assert_eq!(machine.run(&[i]), &[1000 + (i - 8).signum()]);
    }
}
