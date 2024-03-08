use std::{collections::HashMap};

use crate::{combatant::combatant::Combatant, targeting::strategy::target_selection_strategy};
use crate::combat::encounter::IntMutCombatant;

use super::{
    action::Action, attack::Attack, effect::Effect,
    negative_effect::negative_effect::NegativeEffect,
};

#[derive(Debug, Clone)]
pub struct SingleAction {
    pub resource_cost: HashMap<String, u32>,
    pub execution: Execution,
}

#[derive(Debug, Clone)]
pub enum Execution {
    ApplyNegativeEffect(NegativeEffect),
    Attack(Attack),
}

impl Action for SingleAction {
    fn execute(&self, _allies: &[IntMutCombatant], enemies: &[IntMutCombatant]) {
        let strategy = target_selection_strategy();
        match &self.execution {
            Execution::ApplyNegativeEffect(effect) => strategy
                .select_multiple_targets(enemies, effect.number_of_targets())
                .iter()
                .for_each(|enemy| effect.apply(&mut enemy.borrow_mut() as &mut Combatant)),
            Execution::Attack(attack) => strategy
                .select_single_target(enemies)
                .iter()
                .for_each(|enemy| attack.apply(&mut enemy.borrow_mut() as &mut Combatant)),
        }
    }
    fn resource_cost(&self) -> &HashMap<String, u32> {
        &self.resource_cost
    }
}
