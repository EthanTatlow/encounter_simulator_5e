use std::{fmt::Debug, rc::Rc};

use crate::action::action::Action;

#[derive(Default, Clone)]
pub struct ActionSelection {
    pub actions: Vec<Rc<dyn Action>>,
}

impl Debug for ActionSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionSelection")
            .field("actions_len", &self.actions.len())
            .finish()
    }
}

impl ActionSelection {}
