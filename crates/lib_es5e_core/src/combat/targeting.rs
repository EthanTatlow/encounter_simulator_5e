use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

use super::combatant::{Combatant, Target};

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

struct TargetRandomStrategy;

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

struct TargetWeakestStrategy<A> {
    aspect: A,
}

trait SortAspect {
    fn cmp(&self, a: Combatant, b: Combatant) -> Ordering;
}

struct HpAspect;

impl SortAspect for HpAspect {
    fn cmp(&self, a: Combatant, b: Combatant) -> Ordering {
        a.hp().cmp(&b.hp())
    }
}
struct AcAspect;

impl SortAspect for AcAspect {
    fn cmp(&self, a: Combatant, b: Combatant) -> Ordering {
        a.ac().cmp(&b.ac())
    }
}

impl<A> TargetSelectionStrategy for TargetWeakestStrategy<A>
where
    A: SortAspect,
{
    fn select_single_target(
        &self,
        targets: &[Rc<RefCell<Combatant>>],
    ) -> Option<Rc<RefCell<Combatant>>> {
        targets
            .into_iter()
            .filter(|target| target.borrow().is_conscious())
            .min_by_key(|&target| target.borrow().hp())
            .cloned()
    }

    fn select_multiple_targets(
        &self,
        targets: &[Rc<RefCell<Combatant>>],
        max_targets: usize,
    ) -> Vec<Rc<RefCell<Combatant>>> {
        let mut targets_to_sort: Vec<_> = targets
            .iter()
            .filter(|target| target.borrow().is_conscious())
            .cloned()
            .collect();
        targets_to_sort.sort_by(|a, b| a.borrow().hp().cmp(&b.borrow().hp()));
        targets_to_sort.into_iter().take(max_targets).collect()
    }
}

fn get_viable_indices(targets: &[Rc<RefCell<Combatant>>]) -> Vec<usize> {
    targets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.borrow().is_conscious().then(|| i))
        .collect()
}
