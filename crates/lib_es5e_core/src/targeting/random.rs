use std::{cell::RefCell, rc::Rc};

use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

use crate::combat::combatant::Target;

use super::strategy::{get_viable_indices, TargetSelectionStrategy};

pub(super) struct TargetRandomStrategy;

impl<T: Target> TargetSelectionStrategy<T> for TargetRandomStrategy {
    fn select_single_target(&self, targets: &[Rc<RefCell<T>>]) -> Option<Rc<RefCell<T>>> {
        let viable_indices = get_viable_indices(targets);
        viable_indices
            .iter()
            .choose(&mut thread_rng())
            .map(move |&idx| targets[idx].clone())
    }

    fn select_multiple_targets(
        &self,
        targets: &[Rc<RefCell<T>>],
        max_targets: usize,
    ) -> Vec<Rc<RefCell<T>>> {
        let viable_indices: Vec<_> = get_viable_indices(targets);
        let selected: Vec<_> = viable_indices
            .choose_multiple(&mut thread_rng(), max_targets)
            .copied()
            .collect();
        targets
            .iter()
            .enumerate()
            .filter_map(move |(i, p)| selected.contains(&i).then(|| p))
            .cloned()
            .collect()
    }
}
