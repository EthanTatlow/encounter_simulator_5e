use std::{cell::RefCell, rc::Rc};

use crate::combat::combatant::Target;

use super::random::TargetRandomStrategy;

pub trait TargetSelectionStrategy<T: Target> {
    fn select_single_target(&self, targets: &[Rc<RefCell<T>>]) -> Option<Rc<RefCell<T>>>;
    fn select_multiple_targets(
        &self,
        targets: &[Rc<RefCell<T>>],
        max_targets: usize,
    ) -> Vec<Rc<RefCell<T>>>;
}

pub fn target_selection_strategy<T: Target>() -> Box<dyn TargetSelectionStrategy<T>> {
    Box::new(TargetRandomStrategy)
}

pub(super) fn get_viable_indices<T: Target>(targets: &[Rc<RefCell<T>>]) -> Vec<usize> {
    targets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.borrow().is_conscious().then(|| i))
        .collect()
}
