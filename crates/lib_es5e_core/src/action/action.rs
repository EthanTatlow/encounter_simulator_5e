use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::combatant::combatant::Combatant;

pub trait Action {
    fn execute(&self, _allies: &[Rc<RefCell<Combatant>>], enemies: &[Rc<RefCell<Combatant>>]);
    fn resource_cost(&self) -> &HashMap<String, u32>;
}
