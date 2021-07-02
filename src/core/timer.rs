use std::thread;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use spin_sleep::LoopHelper;

pub struct Timer {
    delay_timer: u8,
    sound_timer: u8,
}

impl Timer {
    pub fn new() -> Arc<Mutex<Timer>> {
        let t = Arc::new(Mutex::new(Timer {
            sound_timer: 0,
            delay_timer: 0,
        }));

        Timer::init(t.clone());

        t
    }

    fn init(t: Arc<Mutex<Timer>>) {
        thread::spawn(move || {
            let mut timer = t.lock().unwrap();

            let mut loop_helper = LoopHelper::builder()
                .report_interval_s(0.5) // report every half a second
                .build_with_target_rate(60.0);

            loop {
                let delta = loop_helper.loop_start();

                timer.tick(delta);

                loop_helper.loop_sleep();
            }
        });
    }

    fn tick(&mut self, delta: Duration) {

    }

    pub fn set_delay_timer(&mut self, timer: u8) {
        self.delay_timer = timer;
    }

    pub fn set_sound_timer(&mut self, timer: u8) {
        self.sound_timer = timer;
    }
}