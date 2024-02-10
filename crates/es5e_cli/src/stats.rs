use std::sync::{Arc, Mutex};

use lib_es5e_core::{stats::SimpleStats, stats::Stats};

#[derive(Clone)]
pub struct MultiThreadStats {
    stats: Arc<Mutex<SimpleStats>>,
}

impl MultiThreadStats {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(SimpleStats::new())),
        }
    }
}

impl Stats for MultiThreadStats {
    fn record_round(&mut self) {
        self.stats.lock().unwrap().record_round()
    }

    fn record_win(&mut self, nr_survivors: usize) {
        self.stats.lock().unwrap().record_win(nr_survivors)
    }

    fn print(&self, nr_repetitions: usize) {
        self.stats.lock().unwrap().print(nr_repetitions)
    }
}
