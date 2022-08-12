use interpreter::ChipState;

mod interpreter;

fn main() {
    let mut vm = ChipState::new(700);

    chip8_base::run(vm);
}
