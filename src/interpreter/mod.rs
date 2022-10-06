use std::time::Duration;

pub struct ChipState {
    memory: [u8; 4096],
    program_counter: u16,
    registers: [u8; 16],
    display: chip8_base::Display,
    stack_pointer: u8,
    stack: [u16; 16],
    speed: Duration,
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
        }
    }
}

impl chip8_base::Interpreter for ChipState {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        Some(self.display)
    }

    fn speed(&self) -> Duration {
        self.speed
    }

    fn buzzer_active(&self) -> bool {
        false
    }
}