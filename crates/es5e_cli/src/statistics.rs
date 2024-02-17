use std::sync::{Arc, Mutex};

use lib_es5e_core::{statistics::BaseStatistics, statistics::Statistics};

#[derive(Clone)]
pub struct MultiThreadStatistics {
    stats: Arc<Mutex<BaseStatistics>>,
}

impl MultiThreadStatistics {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(BaseStatistics::new())),
        }
    }
}

impl Statistics for MultiThreadStatistics {
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
