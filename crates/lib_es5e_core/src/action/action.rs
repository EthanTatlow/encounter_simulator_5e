use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

#[cfg(test)]
use mockall::automock;

use crate::combatant::combatant::Combatant;

#[cfg_attr(test, automock)]
pub trait Action {
    fn execute(&self, _allies: &[Rc<RefCell<Combatant>>], enemies: &[Rc<RefCell<Combatant>>]);
    fn resource_cost(&self) -> &HashMap<String, u32>;
}
