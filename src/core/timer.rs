use std::thread;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::ops::Div;

pub struct Timer {
    delay_timer: u8,
    sound_timer: u8,
    time_start: Instant,
    pub frame_count: usize,
}

impl Timer {
    pub fn new() -> Arc<Mutex<Timer>> {
        let t = Arc::new(Mutex::new(Timer {
            sound_timer: 0,
            delay_timer: 0,
            frame_count: 0,
            time_start: Instant::now(),
        }));

        Timer::init(t.clone());

        t
    }

    fn init(t: Arc<Mutex<Timer>>) {
        thread::spawn(move || {
            let mut timer = t.lock().unwrap();

            let mut instant = Instant::now();
            let mut time_limit = Duration::from_millis(1000 / 60 as u64);

            loop {
                let mut elapsed_time = instant.elapsed();

                while time_limit > elapsed_time {
                    elapsed_time = instant.elapsed();
                }

                instant = Instant::now();

                timer.tick();
            }
        });
    }

    fn tick(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);

        let fps = self.frame_count as f32 / self.time_start.elapsed().as_secs() as f32;

        println!("fps {}", fps);
    }

    pub fn set_delay_timer(&mut self, timer: u8) {
        self.delay_timer = timer;
    }

    pub fn set_sound_timer(&mut self, timer: u8) {
        self.sound_timer = timer;
    }
}