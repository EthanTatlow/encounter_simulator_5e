use lib_es5e_core::{combat::encounter::Encounter, stats::Stats};
use loader::load_combatants_from_file;
use rayon::prelude::*;
use stats::MultiThreadStats;
use std::path::Path;

use clap::Parser;

mod loader;
mod stats;

/// Combat encounter simulator for DnD 5e to simulate
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to repeat the simulation
    #[arg(short, long, default_value_t = 10000)]
    repetitions: usize,
    /// Path to file containing enemies
    #[arg(short, long, default_value = "test_data/enemies.yaml")]
    enemies_yaml_path: String,
    /// Path to file containing players
    #[arg(short, long, default_value = "test_data/players.yaml")]
    players_yaml_path: String,
}

impl Args {
    fn load_encounter(&self) -> Encounter {
        let players = load_combatants_from_file(Path::new(self.players_yaml_path.as_str()));
        let enemies = load_combatants_from_file(Path::new(self.enemies_yaml_path.as_str()));

        Encounter::new(players, enemies)
    }
}

fn main() {
    let stats = MultiThreadStats::new();
    let args = Args::parse();
    let repetitions = args.repetitions;
    let encounter = args.load_encounter();
    (0..repetitions)
        .into_par_iter()
        .for_each(|_| encounter.run(&mut stats.clone()));

    stats.print(repetitions);
}
