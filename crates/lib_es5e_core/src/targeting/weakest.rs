use std::cell::RefCell;

use std::rc::Rc;

use std::cmp::Ordering;

use crate::combatant::combatant::Combatant;

use super::strategy::TargetSelectionStrategy;

pub(crate) struct TargetWeakestStrategy<A> {
    pub(crate) aspect: A,
}

pub(crate) trait SortAspect {
    type KeyType: Ord;
    fn key(&self, target: &Combatant) -> Self::KeyType;
    fn cmp(&self, a: &Combatant, b: &Combatant) -> Ordering {
        return self.key(a).cmp(&self.key(b));
    }
}

pub(crate) struct HpAspect;

impl SortAspect for HpAspect {
    type KeyType = u32;
    fn key(&self, target: &Combatant) -> Self::KeyType {
        return target.hp();
    }
}

pub(crate) struct AcAspect;

impl SortAspect for AcAspect {
    type KeyType = i16;
    fn key(&self, target: &Combatant) -> Self::KeyType {
        return target.ac();
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
        targets_to_sort.sort_by(|a, b| self.aspect.cmp(&a.borrow(), &b.borrow()));
        targets_to_sort.into_iter().take(max_targets).collect()
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use rand::{seq::SliceRandom, thread_rng};

    use crate::{
        combat::action_selection::ActionSelection,
        combatant::{combatant::Combatant, defences::save::SaveModifiers},
        targeting::{
            strategy::TargetSelectionStrategy,
            weakest::{AcAspect, HpAspect, TargetWeakestStrategy},
        },
    };

    #[test]
    fn select_weakest_hp_discounting_unconscious() {
        let mut targets: Vec<_> = (0..5)
            .into_iter()
            .map(|hp| {
                Rc::new(RefCell::new(Combatant::new(
                    hp,
                    10,
                    SaveModifiers::default(),
                    ActionSelection::default(),
                )))
            })
            .collect();
        targets.shuffle(&mut thread_rng());

        let sut = TargetWeakestStrategy { aspect: HpAspect };
        let selected = sut.select_single_target(&targets);

        assert_eq!(selected.unwrap().borrow().hp(), 1);
    }

    #[test]
    fn select_multiple_weakest_ac() {
        let mut targets: Vec<_> = (1..5)
            .into_iter()
            .map(|ac| {
                Rc::new(RefCell::new(Combatant::new(
                    42,
                    ac,
                    SaveModifiers::default(),
                    ActionSelection::default(),
                )))
            })
            .collect();

        targets.shuffle(&mut thread_rng());

        let sut = TargetWeakestStrategy { aspect: AcAspect };
        let selected = sut.select_multiple_targets(&targets, 2);

        assert_eq!(2, selected.len());
        assert_eq!(selected.get(0).unwrap().borrow().ac(), 1);
        assert_eq!(selected.get(1).unwrap().borrow().ac(), 2);
    }
}
