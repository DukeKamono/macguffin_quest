use std::time::Duration;

#[derive(Default, Clone, Copy)]
pub struct CountDownTimer {
    time_left: Duration,
}

impl CountDownTimer {
    pub fn new(time_left: Duration) -> CountDownTimer {
        CountDownTimer {
            time_left,
        }
    }

    pub fn update(&mut self, delta: Duration) -> Duration {
        if self.time_left >= delta {
            self.time_left -= delta;
        } else {
            self.time_left = Duration::new(0u64, 0u32);
        }
        self.time_left
    }

    pub fn has_elapsed(&self) -> bool {
        self.time_left > Duration::new(0u64, 0u32)
    }

    pub fn set(&mut self, new_time: Duration) {
        self.time_left = new_time
    }

    pub fn remaining(&self) -> Duration {
        self.time_left
    }
}