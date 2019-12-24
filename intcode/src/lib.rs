use std::{fmt, fs, path};

#[derive(Clone)]
pub struct Machine {
    pub inputs: Vec<isize>,
    pub outputs: Vec<isize>,
    pub mem: Vec<isize>,
    pub pc: usize,
    pub base: isize,
}

impl fmt::Debug for Machine {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "\npc: {} base: {}", self.pc, self.base)?;
        writeln!(fmt, "inputs: {:?}", self.inputs)?;
        writeln!(fmt, "outputs: {:?}", self.outputs)?;
        for i in 0..self.mem.len() {
            if i % 20 == 0 {
                if i > 0 {
                    write!(fmt, "\n")?;
                }
                write!(fmt, "{:04} ", i)?;
            }
            if i % 5 == 0 {
                write!(fmt, "| ")?;
            }
            if self.pc == i {
                write!(fmt, "[{:5}]", self.mem[i])?;
            } else {
                write!(fmt, " {:5} ", self.mem[i])?;
            }
        }
        write!(fmt, "\n")
    }
}

impl Machine {
    pub fn new(mem: Vec<isize>) -> Machine {
        Machine {
            mem,
            base: 0,
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

    pub fn load(&self, at: usize) -> &isize {
        self.mem.get(at).unwrap_or(&0)
    }

    pub fn store(&mut self, at: usize, v: isize) {
        while at >= self.mem.len() {
            self.mem.push(0)
        }
        self.mem[at] = v
    }

    pub fn imm(&self, offset: usize) -> &isize {
        self.load(self.pc + offset as usize)
    }

    pub fn arg(&self, arg: usize) -> &isize {
        let opcode = *self.imm(0) as usize;
        let mode = opcode / 10usize.pow(arg as u32 + 1) % 10;
        let imm = self.imm(arg);
        let v = match mode {
            0 => self.load(*imm as usize),
            1 => imm,
            2 => self.load((self.base + imm) as usize),
            s => panic!("unimplemented addressing mode {}", s),
        };
        v

    }

    pub fn arg_store(&mut self, arg: usize, v: isize) {
        let opcode = *self.imm(0) as usize;
        let mode = opcode / 10usize.pow(arg as u32 + 1) % 10;
        let imm = self.imm(arg);
        let at = match mode {
            0 => *imm as usize,
            2 => (self.base + imm) as usize,
            s => panic!("unimplemented addressing mode {}", s),
        };
        self.store(at, v)
    }

    pub fn done(&self) -> bool {
        self.imm(0) == &99
    }

    pub fn waiting(&self) -> bool {
        self.imm(0) % 100 == 3 && self.inputs.len() == 0
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
                self.outputs.push(*self.arg(1));
                self.pc += 2;
            }
            5 => {
                if *self.arg(1) == 0 {
                    self.pc += 3;
                } else {
                    self.pc = *self.arg(2) as usize;
                }
            }
            6 => {
                if *self.arg(1) != 0 {
                    self.pc += 3;
                } else {
                    self.pc = *self.arg(2) as usize;
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
            9 => {
                self.base += *self.arg(1);
                self.pc += 2;
            }
            x => panic!("unknown instruction {}", x),
        }
    }

    pub fn run(&mut self, inputs: &[isize]) -> &[isize] {
        self.inputs.extend(inputs);
        while !self.done() {
            self.step();
        }
        &*self.outputs
    }

    pub fn steps(&mut self) {
        while !self.done() && !self.waiting() {
            self.step();
        }
    }

    pub fn repl(&mut self) {
        use std::io::{BufRead, BufReader, Write};
        let mut input = BufReader::new(std::io::stdin());
        while !self.done() {
            for o in self.outputs.drain(..) {
                if o < 128 {
                    std::io::stdout().write(&[o as u8]).unwrap();
                } else {
                    dbg!(o);
                }
            }
            if self.waiting() {
                let mut line = String::new();
                input.read_line(&mut line).unwrap();
                self.inputs.extend(line.bytes().map(|b| b as isize));
            }
            self.step();
        }
    }
}

#[test]
pub fn test_day02() {
    let mut machine = Machine::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    machine.run(&[]);
    assert_eq!(machine.load(0), &3500);
}

#[test]
pub fn test_day05_imm() {
    let mut machine = Machine::new(vec![1002, 4, 3, 4, 33]);
    machine.run(&[]);
    assert_eq!(machine.load(4), &99);
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

#[test]
pub fn test_day09_rel() {
    let prg = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut machine = Machine::new(prg.to_vec());
    machine.run(&[]);
    assert_eq!(machine.outputs, prg);
}

#[test]
pub fn test_day09_16digits() {
    let prg = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let mut machine = Machine::new(prg.to_vec());
    machine.run(&[]);
    assert_eq!(machine.outputs.len(), 1);
    assert_eq!(format!("{}", machine.outputs[0]).len(), 16);
}

#[test]
pub fn test_day09_bignum() {
    let prg = vec![104,1125899906842624,99];
    let mut machine = Machine::new(prg.to_vec());
    machine.run(&[]);
    assert_eq!(machine.outputs, vec!(1125899906842624));
}
