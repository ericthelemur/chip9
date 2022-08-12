use interpreter::ChipState;

mod interpreter;

fn main() {
    let vm = ChipState::new(700);
    if let Ok(vml) = vm.load("roms/uwcs.ch8") {
        chip8_base::run(vml);
    } else {
        println!("ROM does not exist.")
    }
}
