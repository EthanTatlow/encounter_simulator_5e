use std::cmp::min;

use rand::{seq::SliceRandom, thread_rng};

use crate::character::character::Character;

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
            let targets = select_targets(effect.number_of_targets(), targets);
            targets.for_each(|target| effect.apply(target));
        });
    }
}

fn select_targets(
    max_targets: u8,
    targets: &mut [Character],
) -> impl Iterator<Item = &mut Character> {
    let viable_targets: Vec<&mut Character> = targets.iter_mut().filter(|c| !c.is_dead()).collect();
    let nr_targets = min(max_targets as usize, viable_targets.len());
    let selected_indices = get_shuffled_indices(viable_targets.len(), nr_targets);

    return viable_targets
        .into_iter()
        .enumerate()
        .filter(move |(i, _)| selected_indices.contains(i))
        .map(|(_, c)| c);
}

fn get_shuffled_indices(max_index: usize, nr_to_select: usize) -> Vec<usize> {
    let mut available_indices = Vec::from_iter(0..max_index);
    available_indices.shuffle(&mut thread_rng());
    available_indices
        .iter()
        .take(nr_to_select)
        .copied()
        .collect()
}
