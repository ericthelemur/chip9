use interpreter::ChipState;

mod interpreter;

fn main() {
    let mut vm = ChipState::new();

    chip8_base::run(vm);
}
