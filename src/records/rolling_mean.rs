use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;
use std::collections::VecDeque;

/// A rolling mean over a sliding time window.
#[derive(Resource, Debug, Clone)]
pub struct RollingMean {
    /// Horizon length in virtual seconds (sliding window).
    pub window: f64,
    /// Time-ordered event timestamps (seconds since startup in `Time<Virtual>`).
    timestamps: VecDeque<f64>,
}

impl RollingMean {
    pub fn new(window: f64) -> Self {
        Self {
            window: window,
            timestamps: VecDeque::<f64>::new(),
        }
    }

    /// Number of events currently within the window.
    pub fn count(&self) -> usize {
        self.timestamps.len()
    }

    /// Average events per simulated second across the window.
    pub fn avg(&self) -> f64 {
        self.count() as f64 / self.window
    }

    /// Push one new event occurrence (at `t` seconds).
    pub fn push(&mut self, t: f64) {
        self.timestamps.push_back(t);
    }

    /// Drop old events outside the horizon.
    pub fn prune(&mut self, now: f64) {
        let cutoff = now - self.window;
        while let Some(&oldest) = self.timestamps.front() {
            if oldest < cutoff {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }
    }
}
