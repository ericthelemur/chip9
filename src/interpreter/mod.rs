use std::time::Duration;

pub struct ChipState {
    memory: [u8; 4096],
    program_counter: u16,
    registers: [u8; 16],
    display: chip8_base::Display,
    stack_pointer: u8,
    stack: [u16; 16],

    speed: Duration
    // ... there will be more
}

impl ChipState {
    pub fn new(freq: u32) -> Self {
        Self { 
            memory: [0; 4096],
            registers: [0; 16], 
            program_counter: 0x200,
            display: [[0; 64]; 32],
            stack_pointer: 0,
            stack: [0; 16],
            speed: Duration::from_secs_f64(1_f64 / freq as f64),
        }
    }

    fn fetch(&mut self) -> u16 {
        dbg!(&self.program_counter);
        let instruction = u16::from_be_bytes([
            self.memory[self.program_counter as usize],
            self.memory[(self.program_counter + 1) as usize],
        ]);
        self.program_counter += 2;
        self.program_counter &= 0x0fff;
        dbg!(&instruction);
        instruction
    }

    //break a u16 into its nibbles
    fn nibbles(n: u16) -> (u8, u8, u8, u8) {
        let n3 = (n >> 12) as u8;
        let n2 = ((n >> 8) & 0b1111) as u8;
        let n1 = ((n >> 4) & 0b1111) as u8;
        let n0 = (n & 0b1111) as u8;
        // Just for debug checking
        let nibbles = format!("{:#X} {:#X} {:#X} {:#X}", n3, n2, n1, n0);
        dbg!(nibbles);

        (n3, n2, n1, n0)
    }

    fn execute(&mut self, instruction: u16) {
        let nbs = Self::nibbles(instruction);
        match nbs {
            // 0000 NOP: Nothing
            (0x0, 0x0, 0x0, 0x0) => (),
            // 00EE RET: Return from subroutine
            (0x0, 0x0, 0xE, 0xE) => {
                self.program_counter = self.stack[self.stack_pointer as usize];
                self.stack_pointer -= 1;
            },
            // 8xy2 AND Vx, Vy: Set Vx = Vx AND Vy.
            (0x8, x, y, 0x2) => {
                self.registers[x as usize] &= self.registers[y as usize];
            }
            _ => panic!("Instruction either doesn't exist or hasn't been implemented yet"),
        }
    }

    
}

impl chip8_base::Interpreter for ChipState {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        let instr = self.fetch();
        self.execute(instr);
        return Some(self.display);
    }

    fn speed(&self) -> std::time::Duration {
        return self.speed;
    }

    fn buzzer_active(&self) -> bool {
        return false;
    }
}