pub mod attack;
pub mod character;
pub mod combat;
pub mod loader;
pub mod utils;

use std::path::Path;

use combat::participant::Participant;
use loader::load_participants_from_file;

use crate::combat::{participant::Damageable, round};

fn main() {
    let repetitions = 10000;

    use std::time::Instant;
    let now = Instant::now();

    let mut players_win_count = 0;
    let mut nr_rounds_sum = 0;

    let players_orig = get_players();
    let enemies_orig = get_enemies();

    for _ in 0..repetitions {
        let mut players = players_orig.to_vec();
        let mut enemies = enemies_orig.to_vec();

        let mut nr_rounds = 0;
        loop {
            nr_rounds += 1;
            round::run_round(&mut players, &mut enemies);
            if players.iter().all(|c| !c.is_conscious()) {
                break;
            }
            if enemies.iter().all(|c| !c.is_conscious()) {
                players_win_count += 1;
                break;
            }
        }
        nr_rounds_sum += nr_rounds;
    }

    println!(
        "Players win {} % of the time",
        players_win_count as f32 / repetitions as f32 * 100.0
    );
    println!(
        "Average number of rounds: {}",
        nr_rounds_sum as f32 / repetitions as f32
    );

    let elapsed = now.elapsed();
    println!("Program duration: {:.2?}", elapsed);
}

fn get_enemies() -> Vec<Participant> {
    return load_participants_from_file(Path::new("data/enemies.yaml"));
}

fn get_players() -> Vec<Participant> {
    return load_participants_from_file(Path::new("data/players.yaml"));
}
