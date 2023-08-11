use rand::Rng;

use crate::{attack::attack::HitResult, character::character::Character}; // Assuming you have a module named "character" with the Character struct

pub fn run_round(players: &mut [Character], enemies: &mut [Character]) {
    // simplifying assumption: first players, then enemies. no initiative
    attack(players, enemies);
    attack(enemies, players);
}

fn attack(attackers: &[Character], targets: &mut [Character]) {
    attackers
        .iter()
        .filter(|c| !c.is_dead())
        .for_each(|c| attack_target(c, targets));
}

fn attack_target(attacker: &Character, target_group: &mut [Character]) {
    if !target_group.is_empty() {
        let target_idx = rand::thread_rng().gen_range(0..target_group.len());
        let (hit_result, total_damage) =
            attacker.roll_attack_with_damage(target_group[target_idx].ac() as i16);

        if hit_result != HitResult::Miss {
            target_group[target_idx].take_damage(total_damage);
        }
    }
}
