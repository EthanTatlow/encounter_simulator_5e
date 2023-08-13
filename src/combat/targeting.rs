use rand::{seq::SliceRandom, thread_rng};

use super::participant::{Damageable, Participant};

pub fn select_random_target(targets: &mut [Participant]) -> Option<&mut Participant> {
    let viable_indices = get_viable_indices(targets);
    let idx = viable_indices.choose(&mut thread_rng());
    if let Some(&idx) = idx {
        targets.get_mut(idx)
    } else {
        None
    }
}

pub fn select_random_targets(
    max_targets: usize,
    targets: &mut [Participant],
) -> impl Iterator<Item = &mut Participant> {
    let viable_indices = get_viable_indices(targets);
    let selected: Vec<usize> = viable_indices
        .choose_multiple(&mut thread_rng(), max_targets)
        .copied()
        .collect();

    targets
        .iter_mut()
        .enumerate()
        .filter_map(move |(i, p)| selected.contains(&i).then(|| p))
}

fn get_viable_indices(targets: &[Participant]) -> Vec<usize> {
    targets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.is_conscious().then(|| i))
        .collect()
}
