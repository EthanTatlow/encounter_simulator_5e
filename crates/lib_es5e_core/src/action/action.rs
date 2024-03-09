use crate::combat::encounter::IntMutCombatant;
#[cfg(test)]
use mockall::automock;

use crate::combatant::state::ResourceCosts;

#[cfg_attr(test, automock)]
pub trait Action {
    fn execute(&self, _allies: &[IntMutCombatant], enemies: &[IntMutCombatant]);
    fn resource_costs(&self) -> &ResourceCosts;
}
