use interpreter::ChipState;

mod interpreter;

fn main() {
    let vm = ChipState::new(700);

    chip8_base::run(vm);
}