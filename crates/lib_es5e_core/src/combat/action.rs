use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    attack::{attack::Attack, save_based::SaveBasedAttack},
    targeting::strategy::target_selection_strategy,
};

use crate::character::effect::NegativeEffect;

use super::combatant::Combatant;

#[derive(Debug, Clone)]
pub enum Action {
    MultiAction(Vec<Action>),
    SingleAttack(Attack),
    MultiAttack(Vec<Attack>),
    SaveBasedAttack(SaveBasedAttack),
}

impl Action {
    pub fn execute(&self, _allies: &[Rc<RefCell<Combatant>>], enemies: &[Rc<RefCell<Combatant>>]) {
        match self {
            Action::MultiAttack(atks) => execute_attacks(atks, enemies),
            Action::SaveBasedAttack(atk) => execute_save_based_attack(atk, enemies),
            Action::SingleAttack(atk) => select_target_and_attack(atk, enemies),
            Action::MultiAction(actions) => actions
                .iter()
                .for_each(|action| action.execute(_allies, enemies)),
        }
    }
}

// TODO: target selection strategy configurable
fn select_target_and_attack(atk: &Attack, enemies: &[Rc<RefCell<Combatant>>]) {
    let strategy = target_selection_strategy();
    if let Some(target) = strategy.select_single_target(enemies) {
        //let mut target = target.clone().borrow().to_owned();
        atk.apply::<Combatant>(&mut target.borrow_mut());
    }
}

fn execute_attacks(atks: &Vec<Attack>, enemies: &[Rc<RefCell<Combatant>>]) {
    atks.iter()
        .for_each(|atk| select_target_and_attack(atk, enemies))
}

fn execute_save_based_attack(atk: &SaveBasedAttack, enemies: &[Rc<RefCell<Combatant>>]) {
    let strategy = target_selection_strategy();
    let targets_iter = strategy.select_multiple_targets(enemies, atk.nr_targets() as usize);
    targets_iter
        .iter()
        .for_each(|enemy| atk.apply::<Combatant>(&mut enemy.borrow_mut()));
}
