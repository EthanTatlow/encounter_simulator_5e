use super::{
    action::Action,
    participant::{ActiveParticipant, Participant},
};

pub fn run_round(players: &mut [Participant], enemies: &mut [Participant]) {
    // simplifying assumption: first players, then enemies. no initiative
    take_actions(players, enemies);
    take_actions(enemies, players);
}

fn take_actions(attackers: &mut [Participant], targets: &mut [Participant]) {
    let actions: Vec<Action> = attackers.iter_mut().map(|a| a.take_action()).collect();
    actions.iter().for_each(|a| a.execute(attackers, targets));
}
