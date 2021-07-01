pub mod core;

use core::Chip8;
use console_engine::{pixel, Color, KeyCode, MouseButton};

fn main() {
    let mut engine = console_engine::ConsoleEngine::init_fill(30).unwrap();

    let mut chip8 = Chip8::new();

    loop {
        engine.wait_frame();
        engine.check_resize();

        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }

        chip8.engine_tick(&mut engine);
    }
}
