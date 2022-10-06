use interpreter::ChipState;

mod interpreter;

fn main() {
    let vm = ChipState::new();

    chip8_base::run(vm);
}