use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use crate::{combatant::combatant::Combatant, targeting::strategy::target_selection_strategy};

use super::{attack::Attack, effect::Effect, negative_effect::negative_effect::NegativeEffect};

#[derive(Debug, Clone, Default)]
pub enum Action {
    Multi(Vec<SingleAction>),
    Single(SingleAction),
    #[default]
    None,
}

impl Action {
    pub fn resource_cost() {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum SingleAction {
    ApplyNegativeEffect(NegativeEffect),
    Attack(Attack),
}

impl SingleAction {
    pub fn execute(&self, _allies: &[Rc<RefCell<Combatant>>], enemies: &[Rc<RefCell<Combatant>>]) {
        let strategy = target_selection_strategy();
        match self {
            Self::ApplyNegativeEffect(effect) => strategy
                .select_multiple_targets(enemies, effect.number_of_targets())
                .iter()
                .for_each(|enemy| effect.apply(&mut enemy.borrow_mut() as &mut Combatant)),
            Self::Attack(attack) => strategy
                .select_single_target(enemies)
                .iter()
                .for_each(|enemy| attack.apply(&mut enemy.borrow_mut() as &mut Combatant)),
        }
    }
}

impl Action {
    pub fn execute(&self, _allies: &[Rc<RefCell<Combatant>>], enemies: &[Rc<RefCell<Combatant>>]) {
        match self {
            Action::Single(action) => action.execute(_allies, enemies),
            Action::Multi(actions) => actions
                .iter()
                .for_each(|action| action.execute(_allies, enemies)),
            Action::None => (),
        }
    }
}
