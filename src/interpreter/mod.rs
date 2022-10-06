use std::time::Duration;

pub struct ChipState {
    memory: [u8; 4096],
    program_counter: u16,
    registers: [u8; 16],
    display: chip8_base::Display,
    stack_pointer: u8,
    stack: [u16; 16],
    speed: Duration,
    index: u16,
    // ... there will be more
}

impl ChipState {
    pub fn new(clock_freq: u32) -> Self {
        Self {
            memory: [0; 4096],
            registers: [0; 16],
            program_counter: 0x200,
            display: [[chip8_base::Pixel::default(); 64]; 32],
            stack_pointer: 0,
            stack: [0; 16],
            speed: Duration::from_secs_f64(1_f64 / clock_freq as f64),
            index: 0,
        }
    }

    pub fn load(mut self, filename: &str) -> std::io::Result<Self> {
        let program = std::fs::read(filename)?;
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program);
        Ok(self)
    }

    fn fetch(&mut self) -> u16 {
        let instruction = u16::from_be_bytes([
            self.memory[self.program_counter as usize],
            self.memory[(self.program_counter + 1) as usize],
        ]);
        log::info!("Instruction {} found at  PC {}", instruction, self.program_counter);
        self.program_counter += 2;
        instruction
    }

    //break a u16 into its nibbles
    fn nibbles(n: u16) -> (u8, u8, u8, u8) {
        let n3 = ( n >> 12)          as u8;
        let n2 = ((n >> 8) & 0b1111) as u8;
        let n1 = ((n >> 4) & 0b1111) as u8;
        let n0 = ( n       & 0b1111) as u8;
        (n3, n2, n1, n0)
    }

    fn nnn(instruction: u16) -> u16 {
        instruction & 0x0FFF
    }

    fn kk(instruction: u16) -> u8 {
        (instruction & 0x00FF) as u8
    }

    fn execute(&mut self, instruction: u16) -> Option<chip8_base::Display> {
        match Self::nibbles(instruction) {
            // 0000 NOP: Nothing
            (0x0, 0x0, 0x0, 0x0) => (),
            // 00E0 CLS: Clears the display
            (0x0, 0x0, 0xE, 0x0) => {
                self.display = [[chip8_base::Pixel::default(); 64]; 32];
                return Some(self.display);
            },
            // 1nnn JP addr: Jump to location nnn
            (0x1, _, _, _) => self.program_counter = Self::nnn(instruction),
            // 6xkk LD Vx, byte: Set Vx = kk.
            (0x6, x, _, _) => self.registers[x as usize] = Self::kk(instruction),
            // 7xkk ADD Vx, byte: Set Vx = Vx + kk.
            (0x7, x, _, _) => {
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(Self::kk(instruction));
            }
            // Annn LD I, addr: Set I = nnn.
            (0xA,_,_,_) => self.index = Self::nnn(instruction),
            // Dxyn DRW Vx, Vy, nibble: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            (0xD, x, y, n) => {
                let tlx = self.registers[x as usize] % 64;
                let tly = self.registers[y as usize] % 32;
                self.registers[0xF] = 0;
                let ind = self.index as usize;
                let sprite = &self.memory[ind..(ind + n as usize)];

                for (i, row) in sprite.iter().enumerate() {
                    let pxy = tly + i as u8;
                    if pxy > 31 {
                        break;
                    }
                    
                    for j in 0..8 {
                        let pxx = tlx + j;
                        if pxx > 63 {
                            break;
                        }
                        let old_px = &mut self.display[pxy as usize][pxx as usize];
                        let mask = 2_u8.pow(7 - j as u32);
                        let new_u8 = (row & mask) >> (7 - j);
                        let new_px: chip8_base::Pixel = new_u8.try_into().unwrap();
                        if (new_px & *old_px).into() { // if collision
                            self.registers[0xF] = 1 
                        }
                        *old_px ^= new_px;
                    }
                }
                return Some(self.display)
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

    fn speed(&self) -> Duration {
        self.speed
    }

    fn buzzer_active(&self) -> bool {
        false
    }
}