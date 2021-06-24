use console_engine::ConsoleEngine;

use crate::chip8::memory::Memory;
use crate::chip8::cpu::CPU;
use crate::chip8::timer::Timer;
use crate::chip8::keyboard::Keyboard;

pub struct Chip8 {
    memory: Memory,
    cpu: CPU,
    timer: Timer,
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

    pub fn load_room(&self, data: &Vec<u8>) {

    }

    pub fn engine_tick(&self, mut engine: &mut ConsoleEngine) {
        engine.clear_screen();



        engine.draw();
    }
}