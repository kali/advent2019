#[derive(Debug)]
pub struct Machine {
    pub memory: Vec<isize>,
    pub pc: usize,
}

impl Machine {
    pub fn new(memory: Vec<isize>) -> Machine {
        Machine { memory, pc: 0 }
    }

    pub fn imm(&self, offset: usize) -> isize {
        self.memory[self.pc + offset as usize]
    }

    pub fn op_load(&self, offset: usize) -> isize {
        self.memory[self.imm(offset) as usize]
    }

    pub fn op_store(&mut self, offset: usize, v: isize) {
        let offset = self.imm(offset);
        self.memory[offset as usize] = v;
    }

    pub fn run(&mut self, inputs: &[isize]) -> Vec<isize> {
        loop {
            match self.imm(0) {
                1 => {
                    self.op_store(3, self.op_load(1) + self.op_load(2));
                    self.pc += 4;
                }
                2 => {
                    self.op_store(3, self.op_load(1) * self.op_load(2));
                    self.pc += 4;
                }
                99 => break,
                _ => panic!(),
            }
        }
        vec![]
    }
}

#[test]
pub fn test_day02() {
    let mut machine = Machine::new(vec!(1,9,10,3,2,3,11,0,99,30,40,50));
    machine.run(&[]);
    assert_eq!(machine.memory[0], 3500);
}
