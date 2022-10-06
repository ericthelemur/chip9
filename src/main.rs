use interpreter::ChipState;

mod interpreter;

fn main() {
    env_logger::init();

    let vm = ChipState::new(700)
        .load("roms/uwcs.ch8").unwrap();

    chip8_base::run(vm);
}