use crate::stats::Stats;

use super::{
    action::Action,
    participant::{ActiveParticipant, Damageable, Participant},
};

pub struct Encounter {
    players: Vec<Participant>,
    enemies: Vec<Participant>,
}

impl Encounter {
    pub fn new(players: Vec<Participant>, enemies: Vec<Participant>) -> Encounter {
        Encounter { players, enemies }
    }

    pub fn run(&self, stats: &mut Stats) {
        let mut players = self.players.to_vec();
        let mut enemies = self.enemies.to_vec();

        loop {
            run_round(&mut players, &mut enemies);
            stats.record_round();
            if players.iter().all(|p| !p.is_conscious()) {
                break;
            }
            if enemies.iter().all(|e| !e.is_conscious()) {
                let nr_survivors = players.iter().filter(|p| p.is_conscious()).count();
                stats.record_win(nr_survivors);
                break;
            }
        }
    }
}

fn run_round(players: &mut [Participant], enemies: &mut [Participant]) {
    // simplifying assumption: first players, then enemies. no initiative
    take_actions(players, enemies);
    take_actions(enemies, players);
}

pub fn take_actions(attackers: &mut [Participant], targets: &mut [Participant]) {
    let actions: Vec<Action> = attackers.iter_mut().map(|a| a.take_action()).collect();
    actions.iter().for_each(|a| a.execute(attackers, targets));
}
