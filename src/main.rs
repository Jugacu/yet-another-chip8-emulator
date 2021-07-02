use console_engine::{pixel, Color, KeyCode, MouseButton};

pub mod core;

use crate::core::Chip8;

fn main() {
    let mut engine = console_engine::ConsoleEngine::init_fill(30).unwrap();

    let chip8 = Chip8::new();

    loop {
        engine.wait_frame();
        engine.check_resize();

        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }

        chip8.engine_tick(&mut engine);
    }
}
