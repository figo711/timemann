use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub struct FpsCounter {
    start_time: Instant,
    frames: u32,
    pub fps: f64,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            frames: 0,
            fps: 0.0,
        }
    }

    pub fn tick(&mut self) {
        self.frames += 1;
        let now = Instant::now();
        let elapsed = (now - self.start_time).as_secs_f64();
        if elapsed >= 1.0 {
            self.fps = self.frames as f64 / elapsed;
            self.start_time = now;
            self.frames = 0;
        }
    }
}
