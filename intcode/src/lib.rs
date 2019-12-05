use std::{fmt, fs, path};

pub struct Machine {
    pub memory: Vec<isize>,
    pub pc: usize,
}

impl fmt::Debug for Machine {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.memory.len() {
            if self.pc == i {
                write!(fmt, "[{}], ", self.memory[i])?;
            } else {
                write!(fmt, "{}, ", self.memory[i])?;
            }
        }
        Ok(())
    }
}

impl Machine {
    pub fn new(memory: Vec<isize>) -> Machine {
        Machine { memory, pc: 0 }
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

    pub fn run(&mut self, mut inputs: &[isize]) -> Vec<isize> {
        dbg!(&self);
        let mut outputs = vec![];
        loop {
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
                    let address = self.imm(1) as usize;
                    self.memory[address] = inputs[0];
                    inputs = &inputs[1..];
                    self.pc += 2;
                }
                4 => {
                    outputs.push(self.memory[self.imm(1) as usize]);
                    self.pc += 2;
                    break;
                }
                99 => break,
                _ => panic!(),
            }
            dbg!(&self);
        }
        outputs
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
    assert_eq!(machine.run(&[57]), vec!(57));
}
