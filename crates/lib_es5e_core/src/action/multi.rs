use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::combatant::combatant::Combatant;

use super::{action::Action, single::SingleAction};

#[derive(Debug, Clone)]
pub struct MultiAction {
    actions: Vec<SingleAction>,
    resource_cost: HashMap<String, u32>,
}

impl MultiAction {
    pub fn new(actions: Vec<SingleAction>) -> Self {
        let resource_cost = actions.iter().map(SingleAction::resource_cost).fold(
            HashMap::<String, u32>::new(),
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
    fn execute(&self, allies: &[Rc<RefCell<Combatant>>], enemies: &[Rc<RefCell<Combatant>>]) {
        self.actions
            .iter()
            .for_each(|action| action.execute(allies, enemies));
    }

    fn resource_cost(&self) -> &HashMap<String, u32> {
        &self.resource_cost
    }
}
