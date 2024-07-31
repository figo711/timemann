use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub struct Timer {
    pub start: Option<Instant>,
    pub elapsed: Duration,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: None,
            elapsed: Duration::new(0, 0),
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn pause(&mut self) {
        self.elapsed = self.elapsed();
        self.start = None;
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.elapsed = Duration::new(0, 0);
    }

    pub fn elapsed(&self) -> Duration {
        match self.start {
			Some(t1) => {
				return t1.elapsed() + self.elapsed;
			},
			None => {
				return self.elapsed;
			},
		}
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
