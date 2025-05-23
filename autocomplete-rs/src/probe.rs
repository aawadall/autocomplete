use std::time::{Duration, Instant};

/// A trait for performance measurement probes
pub trait Probe {
    /// Start timing an operation
    fn start(&mut self, id: u64);
    /// Stop timing an operation
    fn stop(&mut self, id: u64);
}

/// A no-operation probe that does nothing
#[derive(Debug, Default)]
pub struct NopProbe;

impl Probe for NopProbe {
    fn start(&mut self, _id: u64) {}
    fn stop(&mut self, _id: u64) {}
}

/// A timer probe that measures operation durations
#[derive(Debug)]
pub struct TimerProbe {
    timers: Vec<Timer>,
}

#[derive(Debug, Default, Clone)]
struct Timer {
    start_time: Option<Instant>,
    total_duration: Duration,
}

impl Timer {
    fn new() -> Self {
        Self {
            start_time: None,
            total_duration: Duration::default(),
        }
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    fn stop(&mut self) {
        if let Some(start) = self.start_time {
            self.total_duration += start.elapsed();
            self.start_time = None;
        }
    }

    fn get_duration(&self) -> Duration {
        self.total_duration
    }
}

impl TimerProbe {
    /// Creates a new TimerProbe with the specified number of timers
    pub fn new(num_timers: u64) -> Self {
        Self {
            timers: vec![Timer::new(); num_timers as usize],
        }
    }

    /// Gets the total duration for a specific timer
    pub fn get_duration(&self, id: u64) -> Duration {
        assert!(id < self.timers.len() as u64, "Timer ID out of bounds");
        self.timers[id as usize].get_duration()
    }
}

impl Probe for TimerProbe {
    fn start(&mut self, id: u64) {
        assert!(id < self.timers.len() as u64, "Timer ID out of bounds");
        self.timers[id as usize].start();
    }

    fn stop(&mut self, id: u64) {
        assert!(id < self.timers.len() as u64, "Timer ID out of bounds");
        self.timers[id as usize].stop();
    }
} 