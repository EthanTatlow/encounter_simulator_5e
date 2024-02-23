use std::{cell::RefCell, rc::Rc};

use crate::combatant::combatant::Combatant;

use super::random::TargetRandomStrategy;

pub trait TargetSelectionStrategy {
    fn select_single_target(
        &self,
        targets: &[Rc<RefCell<Combatant>>],
    ) -> Option<Rc<RefCell<Combatant>>>;
    fn select_multiple_targets(
        &self,
        targets: &[Rc<RefCell<Combatant>>],
        max_targets: usize,
    ) -> Vec<Rc<RefCell<Combatant>>>;
}

pub fn target_selection_strategy() -> Box<dyn TargetSelectionStrategy> {
    Box::new(TargetRandomStrategy)
}

pub(super) fn get_viable_indices(targets: &[Rc<RefCell<Combatant>>]) -> Vec<usize> {
    targets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.borrow().is_conscious().then(|| i))
        .collect()
}
