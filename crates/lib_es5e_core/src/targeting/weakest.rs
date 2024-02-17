use std::cell::RefCell;

use std::rc::Rc;

use std::cmp::Ordering;

use crate::targeting::target::Target;

use super::strategy::TargetSelectionStrategy;

pub(crate) struct TargetWeakestStrategy<A> {
    pub(crate) aspect: A,
}

pub(crate) trait SortAspect<T: Target> {
    type KeyType: Ord;
    fn key(&self, target: &T) -> Self::KeyType;
    fn cmp(&self, a: &T, b: &T) -> Ordering {
        return self.key(a).cmp(&self.key(b));
    }
}

pub(crate) struct HpAspect;

impl<T: Target> SortAspect<T> for HpAspect {
    type KeyType = u32;
    fn key(&self, target: &T) -> Self::KeyType {
        return target.hp();
    }
}

pub(crate) struct AcAspect;

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

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use rand::{seq::SliceRandom, thread_rng};

    use crate::targeting::{
        strategy::TargetSelectionStrategy,
        target::{MockTarget, Target},
        weakest::{HpAspect, TargetWeakestStrategy},
    };

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
