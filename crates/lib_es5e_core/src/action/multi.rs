use crate::combat::encounter::IntMutCombatant;


use crate::combatant::{state::ResourceCosts};

use super::{action::Action, single::SingleAction};

#[derive(Debug, Clone)]
pub struct MultiAction {
    actions: Vec<SingleAction>,
    resource_cost: ResourceCosts,
}

impl MultiAction {
    pub fn new(actions: Vec<SingleAction>) -> Self {
        let resource_cost = actions.iter().map(SingleAction::resource_costs).fold(
            ResourceCosts::new(),
            |mut acc, e| {
                acc.extend(e.clone());
                acc
            },
        );
        Self {
            actions,
            resource_cost,
        }
    }
}

impl Action for MultiAction {
    fn execute(&self, allies: &[IntMutCombatant], enemies: &[IntMutCombatant]) {
        self.actions
            .iter()
            .for_each(|action| action.execute(allies, enemies));
    }

    fn resource_costs(&self) -> &ResourceCosts {
        &self.resource_cost
    }
}
