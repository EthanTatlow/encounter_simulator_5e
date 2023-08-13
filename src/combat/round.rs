use crate::character::character::Character;

use super::targeting::select_random_targets;

pub fn run_round(players: &mut [Character], enemies: &mut [Character]) {
    // simplifying assumption: first players, then enemies. no initiative
    attack_with_each_attacker(players, enemies);
    attack_with_each_attacker(enemies, players);
}

fn attack_with_each_attacker(attackers: &[Character], targets: &mut [Character]) {
    attackers
        .iter()
        .filter(|c| !c.is_dead())
        .for_each(|c| attack_targets(c, targets));
}

fn attack_targets(attacker: &Character, targets: &mut [Character]) {
    if !targets.is_empty() {
        attacker.get_effects_on_enemies().iter().for_each(|effect| {
            let targets = select_random_targets(effect.number_of_targets() as usize, targets);
            targets.for_each(|target| effect.apply(target));
        });
    }
}
