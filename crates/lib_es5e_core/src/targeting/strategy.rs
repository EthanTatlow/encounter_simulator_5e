use crate::combat::encounter::IntMutCombatant;

use super::random::TargetRandomStrategy;

pub trait TargetSelectionStrategy {
    fn select_single_target(
        &self,
        targets: &[IntMutCombatant],
    ) -> Option<IntMutCombatant>;
    fn select_multiple_targets(
        &self,
        targets: &[IntMutCombatant],
        max_targets: usize,
    ) -> Vec<IntMutCombatant>;
}

pub fn target_selection_strategy() -> Box<dyn TargetSelectionStrategy> {
    Box::new(TargetRandomStrategy)
}

pub(super) fn get_viable_indices(targets: &[IntMutCombatant]) -> Vec<usize> {
    targets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.borrow().is_conscious().then(|| i))
        .collect()
}

#[cfg(test)]
pub(super) mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::combat::action_selection::ActionSelection;
    use crate::combat::encounter::IntMutCombatant;
    use crate::combatant::combatant::Combatant;
    use crate::combatant::defences::save::SaveModifiers;
    use crate::targeting::strategy::{get_viable_indices, TargetSelectionStrategy};

    pub fn init_identical_test_targets(nr_targets: usize) -> Vec<IntMutCombatant> {
        (0..nr_targets)
            .into_iter()
            .map(|_| {
                Rc::new(RefCell::new(Combatant::new(
                    100,
                    10,
                    0,
                    SaveModifiers::default(),
                    ActionSelection::default(),
                )))
            })
            .collect()
    }

    pub fn test_select_as_many_targets_as_specified(target_strategy: impl TargetSelectionStrategy) {
        let nr_targets = 10;
        let targets = init_identical_test_targets(nr_targets);

        for to_select in 0..nr_targets + 1 {
            let selected = target_strategy.select_multiple_targets(&targets, to_select);

            assert_eq!(to_select, selected.len())
        }
    }

    pub fn test_selecting_too_many_targets_returns_all(target_strategy: impl TargetSelectionStrategy) {
        let nr_targets = 10;
        let max_targets = 11;
        let targets = init_identical_test_targets(nr_targets);

        let selected = target_strategy.select_multiple_targets(&targets, max_targets as usize);

        assert_eq!(nr_targets, selected.len())
    }

    #[test]
    fn get_viable_indices_only_returns_conscious_targets() {
        let nr_targets = 10;
        let nr_conscious = 5;
        assert!(nr_conscious < nr_targets);

        let targets: Vec<_> = (0..nr_targets)
            .into_iter()
            .map(|i| {
                Rc::new(RefCell::new(Combatant::new(
                    if i < nr_conscious { 1 } else { 0 },
                    10,
                    0,
                    SaveModifiers::default(),
                    ActionSelection::default(),
                )))
            })
            .collect();

        let viable_indices = get_viable_indices(&targets);
        assert_eq!(nr_conscious, viable_indices.len());
        assert!(viable_indices.iter().all(|&x| x < nr_conscious));
    }
}
