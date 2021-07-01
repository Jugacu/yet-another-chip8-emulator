use std::thread;

pub struct Timer {
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            sound_timer: 0,
            delay_timer: 0,
        }
    }

    pub fn tick(&self) {
        println!("hi from the spawned thread!");
    }

    pub fn set_delay_timer(&mut self, timer: u8) {
        self.delay_timer = timer;
    }

    pub fn set_sound_timer(&mut self, timer: u8) {
        self.sound_timer = timer;
    }
}