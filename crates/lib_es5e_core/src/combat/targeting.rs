use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

use super::combatant::Target;

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

struct TargetRandomStrategy;

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

struct TargetWeakestStrategy<A> {
    aspect: A,
}

trait SortAspect<T: Target> {
    type KeyType: Ord;
    fn key(&self, target: &T) -> Self::KeyType;
    fn cmp(&self, a: &T, b: &T) -> Ordering {
        return self.key(a).cmp(&self.key(b));
    }
}

struct HpAspect;

impl<T: Target> SortAspect<T> for HpAspect {
    type KeyType = u32;
    fn key(&self, target: &T) -> Self::KeyType {
        return target.hp();
    }
}
struct AcAspect;

impl<T: Target> SortAspect<T> for AcAspect {
    type KeyType = i16;
    fn key(&self, target: &T) -> Self::KeyType {
        return target.ac();
    }
}

impl<A, T> TargetSelectionStrategy<T> for TargetWeakestStrategy<A>
where
    A: SortAspect<T>,
    T: Target,
{
    fn select_single_target(&self, targets: &[Rc<RefCell<T>>]) -> Option<Rc<RefCell<T>>> {
        targets
            .into_iter()
            .filter(|target| target.borrow().is_conscious())
            .min_by_key(|&target| target.borrow().hp())
            .cloned()
    }

    fn select_multiple_targets(
        &self,
        targets: &[Rc<RefCell<T>>],
        max_targets: usize,
    ) -> Vec<Rc<RefCell<T>>> {
        let mut targets_to_sort: Vec<_> = targets
            .iter()
            .filter(|target| target.borrow().is_conscious())
            .cloned()
            .collect();
        targets_to_sort.sort_by(|a, b| self.aspect.cmp(&a.borrow(), &b.borrow()));
        targets_to_sort.into_iter().take(max_targets).collect()
    }
}

fn get_viable_indices<T: Target>(targets: &[Rc<RefCell<T>>]) -> Vec<usize> {
    targets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.borrow().is_conscious().then(|| i))
        .collect()
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use rand::{seq::SliceRandom, thread_rng};

    use crate::combat::combatant::{MockTarget, Target};

    use super::{HpAspect, TargetSelectionStrategy, TargetWeakestStrategy};

    #[test]
    fn test_select_weakest() {
        let mut targets = vec![Rc::new(RefCell::new(MockTarget::new())); 4];
        for (i, target) in targets.iter().enumerate() {
            target.borrow_mut().expect_is_conscious().return_const(true);
            target.borrow_mut().expect_hp().return_const(1 + i as u32);
        }
        targets.shuffle(&mut thread_rng());
        let sut = TargetWeakestStrategy {
            aspect: HpAspect {},
        };
        let a = sut.select_single_target(&targets);
        assert_eq!(a.unwrap().borrow().hp(), 1);
    }

    #[test]
    fn test_select_multiple_weakest() {
        let mut targets: Vec<Rc<RefCell<_>>> = (0..4)
            .into_iter()
            .map(|_| Rc::new(RefCell::new(MockTarget::new())))
            .collect();
        for (i, target) in targets.iter().enumerate() {
            target.borrow_mut().expect_is_conscious().returning(|| true);
            target
                .borrow_mut()
                .expect_hp()
                .returning(move || 1 + i as u32);
        }
        targets.shuffle(&mut thread_rng());

        let sut = TargetWeakestStrategy {
            aspect: HpAspect {},
        };
        let a = sut.select_multiple_targets(&targets, 2);
        assert_eq!(2, a.len());
        assert_eq!(a.get(0).unwrap().borrow().hp(), 1);
        assert_eq!(a.get(1).unwrap().borrow().hp(), 2);
    }
}
