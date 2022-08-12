use std::time::Duration;

pub struct ChipState {
    memory: [u8; 4096],
    program_counter: u16,
    registers: [u8; 16],
    display: chip8_base::Display,
    stack_pointer: u8,
    stack: [u16; 16],
    index: u16,

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
            index: 0,
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
        (n3, n2, n1, n0)
    }

    fn nnn(instruction: u16) -> u16 {
        instruction & 0x0FFF
    }

    fn kk(instruction: u16) -> u8 {
        (instruction & 0x00FF) as u8
    }

    fn execute(&mut self, instruction: u16) -> Option<chip8_base::Display> {
        let nbs = Self::nibbles(instruction);
        match nbs {
            // 0000 NOP: Nothing
            (0x0, 0x0, 0x0, 0x0) => (),
            // 00E0 CLS: Clears the display
            (0x0, 0x0, 0xE, 0x0) => {
                self.display = [[0; 64]; 32];
                return Some(self.display);
            },
            // 1nnn JP addr: Jump to location nnn
            (0x1, _, _, _) => self.program_counter = Self::nnn(instruction),
            // 6xkk LD Vx, byte: Set Vx = kk.
            (0x6, x, _, _) => self.registers[x as usize] = Self::kk(instruction),
            // 7xkk ADD Vx, byte: Set Vx = Vx + kk.
            (0x7, x, _, _) => {
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(Self::kk(instruction));
            },
            // Annn LD I, addr: Set I = nnn.
            (0xA,_,_,_) => self.index = Self::nnn(instruction),
            // Dxyn DRW Vx, Vy, n: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF if collision.
            (0xD, x, y, n) => {
                // Wrap to screen size
                let tlx = self.registers[x as usize] % 64;
                let tly = self.registers[y as usize] % 32;
                self.registers[0xF] = 0;
                let ind = self.index as usize;
                let sprite = &self.memory[ind..(ind + n as usize)]; // Fetch as slice
                
                // Enumerate to get the value (row) and index (i) at once
                for (i, row) in sprite.iter().enumerate() {
                    let pxy = tly + i as u8;
                    if pxy > 31 {   // Stop at edge
                        break;
                    }
                    
                    for j in 0..8 {     // For each bit index
                        let pxx = tlx + j;
                        if pxx > 63 {   // Stop at edge
                            break;
                        }
                        let old_px = &mut self.display[pxy as usize][pxx as usize];     // Fetch old px as reference
                        let mask = 1_u8 << (7 - j as u32);      // Calculate bitmask for bit j
                        let new_px = (row & mask) >> (7 - j);        // Mask and shift to 0 or 1

                        // Check for collision
                        if new_px == 1 && *old_px == 1 {
                            self.registers[0xF] = 1 
                        }
                        *old_px ^= new_px;      // Apply as XOR
                    }
                }
                return Some(self.display);
            },
            _ => panic!("Instruction either doesn't exist or hasn't been implemented yet"),
        };
        None
    }

    
}

impl chip8_base::Interpreter for ChipState {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        let instr = self.fetch();
        self.execute(instr)
    }

    fn speed(&self) -> std::time::Duration {
        return self.speed;
    }

    fn buzzer_active(&self) -> bool {
        return false;
    }
}