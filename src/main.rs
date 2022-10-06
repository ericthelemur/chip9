use interpreter::ChipState;

mod interpreter;

fn main() {
    env_logger::init();

    let vm = ChipState::new(700);

    chip8_base::run(vm);
}