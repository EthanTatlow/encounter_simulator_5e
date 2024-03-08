use std::{cell::RefCell, rc::Rc};

use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

use crate::combatant::combatant::Combatant;

use super::strategy::{get_viable_indices, TargetSelectionStrategy};

pub(super) struct TargetRandomStrategy;

impl TargetSelectionStrategy for TargetRandomStrategy {
    fn select_single_target(
        &self,
        targets: &[Rc<RefCell<Combatant>>],
    ) -> Option<Rc<RefCell<Combatant>>> {
        let viable_indices = get_viable_indices(targets);
        viable_indices
            .iter()
            .choose(&mut thread_rng())
            .map(move |&idx| targets[idx].clone())
    }

    fn select_multiple_targets(
        &self,
        targets: &[Rc<RefCell<Combatant>>],
        max_targets: usize,
    ) -> Vec<Rc<RefCell<Combatant>>> {
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



#[cfg(test)]
mod tests {
    use crate::targeting::random::TargetRandomStrategy;
    use crate::targeting::strategy::tests::{test_select_as_many_targets_as_specified, test_selecting_too_many_targets_returns_all};

    #[test]
    fn selecting_too_many_targets_returns_all() {
        test_selecting_too_many_targets_returns_all(TargetRandomStrategy);
    }

    #[test]
    fn select_as_many_targets_as_specified() {
        test_select_as_many_targets_as_specified(TargetRandomStrategy);
    }
}
