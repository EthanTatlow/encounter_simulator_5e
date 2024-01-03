use std::time::Instant;

pub struct Stats {
    start: Instant,
    players_win_count: usize,
    nr_rounds_sum: usize,
    nr_survivors_sum: usize,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            players_win_count: 0,
            nr_rounds_sum: 0,
            nr_survivors_sum: 0,
        }
    }

    pub fn record_round(&mut self) {
        self.nr_rounds_sum += 1;
    }

    pub fn record_win(&mut self, nr_survivors: usize) {
        self.players_win_count += 1;
        self.nr_survivors_sum += nr_survivors;
    }

    pub fn print(&self, repetitions: usize) {
        println!(
            "Players win {} % of the time",
            self.players_win_count as f32 / repetitions as f32 * 100.0
        );
        println!(
            "Average number of rounds: {}",
            self.nr_rounds_sum as f32 / repetitions as f32
        );
        if self.players_win_count > 0 {
            println!(
                "Average number of survivors on win: {}",
                self.nr_survivors_sum as f32 / self.players_win_count as f32
            );
        }
        println!("Program duration: {:.2?}", self.start.elapsed());
    }
}
