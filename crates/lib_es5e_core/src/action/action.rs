
use std::{collections::HashMap};

#[cfg(test)]
use mockall::automock;
use crate::combat::encounter::IntMutCombatant;



#[cfg_attr(test, automock)]
pub trait Action {
    fn execute(&self, _allies: &[IntMutCombatant], enemies: &[IntMutCombatant]);
    fn resource_cost(&self) -> &HashMap<String, u32>;
}
