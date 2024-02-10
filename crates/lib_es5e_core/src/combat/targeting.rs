use std::{cell::RefCell, rc::Rc};

use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

use super::participant::{Damageable, Participant};

pub trait TargetSelectionStrategy {
    fn select_single_target<'a>(
        &'a self,
        targets: &'a [Rc<RefCell<Participant>>],
    ) -> Option<Rc<RefCell<Participant>>>;
    fn select_multiple_targets<'a>(
        &'a self,
        targets: &'a [Rc<RefCell<Participant>>],
        max_targets: usize,
    ) -> Box<dyn Iterator<Item = Rc<RefCell<Participant>>> + 'a>;
}

pub fn target_selection_strategy() -> Box<dyn TargetSelectionStrategy> {
    Box::new(TargetRandomStrategy)
}

struct TargetRandomStrategy;

impl TargetSelectionStrategy for TargetRandomStrategy {
    fn select_single_target<'a>(
        &'a self,
        targets: &'a [Rc<RefCell<Participant>>],
    ) -> Option<Rc<RefCell<Participant>>> {
        let viable_indices = get_viable_indices(targets);
        viable_indices
            .iter()
            .choose(&mut thread_rng())
            .map(move |&idx| targets[idx].clone())
    }

    fn select_multiple_targets<'a>(
        &'a self,
        targets: &'a [Rc<RefCell<Participant>>],
        max_targets: usize,
    ) -> Box<dyn Iterator<Item = Rc<RefCell<Participant>>> + 'a> {
        let viable_indices: Vec<usize> = get_viable_indices(targets);
        let selected: Vec<usize> = viable_indices
            .choose_multiple(&mut thread_rng(), max_targets)
            .copied()
            .collect();
        Box::new(
            targets
                .iter()
                .enumerate()
                .filter_map(move |(i, p)| selected.contains(&i).then(|| p))
                .cloned(),
        )
    }
}

struct TargetWeakestStrategy;

impl TargetSelectionStrategy for TargetWeakestStrategy {
    fn select_single_target<'a>(
        &'a self,
        targets: &'a [Rc<RefCell<Participant>>],
    ) -> Option<Rc<RefCell<Participant>>> {
        targets
            .into_iter()
            .filter(|target| target.borrow().hp() > 0)
            .min_by_key(|&target| target.borrow().hp())
            .cloned()
    }

    fn select_multiple_targets<'a>(
        &'a self,
        targets: &'a [Rc<RefCell<Participant>>],
        max_targets: usize,
    ) -> Box<dyn Iterator<Item = Rc<RefCell<Participant>>> + 'a> {
        let selected = targets
            .into_iter()
            .filter(|&target| target.borrow().hp() > 0)
            .map(|target| target.clone())
            .take(max_targets);
        Box::new(selected)
    }
}

fn get_viable_indices(targets: &[Rc<RefCell<Participant>>]) -> Vec<usize> {
    targets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.borrow().is_conscious().then(|| i))
        .collect()
}
