use std::time::Duration;

pub struct ChipState {
    memory: [u8; 4096],
    program_counter: u16,
    registers: [u8; 16],
    display: chip8_base::Display,
    stack_pointer: u8,
    stack: [u16; 16],
    // ... there will be more
}

impl ChipState {
    pub fn new() -> Self {
        Self { 
            memory: [0; 4096],
            registers: [0; 16], 
            program_counter: 0x200,
            display: [[0; 64]; 32],
            stack_pointer: 0,
            stack: [0; 16],
        }
    }
}

impl chip8_base::Interpreter for ChipState {
    fn step(&mut self, keys: &chip8_base::Keys) -> Option<chip8_base::Display> {
        return Some(self.display);
    }

    fn speed(&self) -> std::time::Duration {
        return Duration::new(1, 0);
    }

    fn buzzer_active(&self) -> bool {
        return false;
    }
}