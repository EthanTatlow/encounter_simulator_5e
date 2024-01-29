use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

use super::participant::{Damageable, Participant};

pub trait TargetSelectionStrategy {
    fn select_single_target<'a>(
        &'a self,
        targets: &'a mut [Participant],
    ) -> Option<&mut Participant>;
    fn select_multiple_targets<'a>(
        &'a self,
        targets: &'a mut [Participant],
        max_targets: usize,
    ) -> Box<dyn Iterator<Item = &mut Participant> + 'a>;
}

pub fn target_selection_strategy() -> Box<dyn TargetSelectionStrategy> {
    Box::new(TargetRandomStrategy)
}

struct TargetRandomStrategy;

impl TargetSelectionStrategy for TargetRandomStrategy {
    fn select_single_target<'a>(
        &'a self,
        targets: &'a mut [Participant],
    ) -> Option<&mut Participant> {
        let viable_indices = get_viable_indices(targets);
        viable_indices
            .iter()
            .choose(&mut thread_rng())
            .map(move |&idx| &mut targets[idx])
    }

    fn select_multiple_targets<'a>(
        &'a self,
        targets: &'a mut [Participant],
        max_targets: usize,
    ) -> Box<dyn Iterator<Item = &mut Participant> + 'a> {
        let viable_indices: Vec<usize> = get_viable_indices(targets);
        let selected: Vec<usize> = viable_indices
            .choose_multiple(&mut thread_rng(), max_targets)
            .copied()
            .collect();
        Box::new(
            targets
                .iter_mut()
                .enumerate()
                .filter_map(move |(i, p)| selected.contains(&i).then(|| p)),
        )
    }
}

struct TargetWeakestStrategy;

impl TargetSelectionStrategy for TargetWeakestStrategy {
    fn select_single_target<'a>(
        &'a self,
        targets: &'a mut [Participant],
    ) -> Option<&mut Participant> {
        targets
            .iter_mut()
            .filter(|target| target.hp() > 0)
            .min_by_key(|target| target.hp())
    }

    fn select_multiple_targets<'a>(
        &'a self,
        targets: &'a mut [Participant],
        max_targets: usize,
    ) -> Box<dyn Iterator<Item = &mut Participant> + 'a> {
        Box::new(
            targets
                .iter_mut()
                .filter(|target| target.hp() > 0)
                .take(max_targets),
        )
    }
}

fn get_viable_indices(targets: &[Participant]) -> Vec<usize> {
    targets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.is_conscious().then(|| i))
        .collect()
}
