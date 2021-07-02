pub mod memory;
pub mod cpu;
pub mod timer;
pub mod keyboard;

use std::sync::{Arc, Mutex};
use console_engine::ConsoleEngine;

use crate::core::memory::Memory;
use crate::core::cpu::CPU;
use crate::core::timer::Timer;
use crate::core::keyboard::Keyboard;

pub struct Chip8 {
    memory: Memory,
    cpu: CPU,
    timer: Arc<Mutex<Timer>>,
    keyboard: Keyboard
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: Memory::new(),
            cpu: CPU::new(),
            timer: Timer::new(),
            keyboard: Keyboard::new()
        }
    }

    pub fn load_rom(&self, data: &Vec<u8>) {

    }

    pub fn engine_tick(&self, mut engine: &mut ConsoleEngine) {
        engine.clear_screen();



        engine.draw();
    }
}