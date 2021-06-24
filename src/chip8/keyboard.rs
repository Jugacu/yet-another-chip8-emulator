use console_engine::KeyCode;

pub struct Keyboard {
    key_pressed: Option<u8>
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { key_pressed: None }
    }

    pub fn set_key_pressed(&mut self, key: Option<KeyCode>) {
        self.key_pressed = Keyboard::get_chip8_keycode_for(key);
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key_pressed
    }

    fn get_chip8_keycode_for(key: Option<KeyCode>) -> Option<u8> {
        match key {
            Some(KeyCode::Char('1')) => Some(0x1),
            Some(KeyCode::Char('2')) => Some(0x2),
            Some(KeyCode::Char('3')) => Some(0x3),
            Some(KeyCode::Char('4')) => Some(0xC),

            Some(KeyCode::Char('q')) => Some(0x4),
            Some(KeyCode::Char('w')) => Some(0x5),
            Some(KeyCode::Char('e')) => Some(0x6),
            Some(KeyCode::Char('r')) => Some(0xD),

            Some(KeyCode::Char('a')) => Some(0x7),
            Some(KeyCode::Char('s')) => Some(0x8),
            Some(KeyCode::Char('d')) => Some(0x9),
            Some(KeyCode::Char('f')) => Some(0xE),

            Some(KeyCode::Char('z')) => Some(0xA),
            Some(KeyCode::Char('x')) => Some(0x0),
            Some(KeyCode::Char('c')) => Some(0xB),
            Some(KeyCode::Char('v')) => Some(0xF),
            _ => None,
        }
    }
}