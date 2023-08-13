use rand::{seq::SliceRandom, thread_rng};

use crate::character::character::Character;

pub fn select_random_target(targets: &mut [Character]) -> Option<&mut Character> {
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
    targets: &mut [Character],
) -> impl Iterator<Item = &mut Character> {
    let viable_indices = get_viable_indices(targets);
    let selected: Vec<usize> = viable_indices
        .choose_multiple(&mut thread_rng(), max_targets)
        .copied()
        .collect();

    targets
        .iter_mut()
        .enumerate()
        .filter_map(move |(i, c)| selected.contains(&i).then(|| c))
}

fn get_viable_indices(targets: &[Character]) -> Vec<usize> {
    targets
        .iter()
        .enumerate()
        .filter_map(|(i, c)| (!c.is_dead()).then(|| i))
        .collect()
}
