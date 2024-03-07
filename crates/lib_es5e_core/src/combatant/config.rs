use std::rc::Rc;

use crate::action::{action::Action, multi::MultiAction, single::SingleAction};

use super::{combatant::Combatant, state::Resources, stats::CombatantStats};

#[derive(Clone, Debug)]
pub struct CombatantConfig {
    pub resources: Resources,
    pub actions: Vec<ActionType>,
    pub stats: CombatantStats,
}

#[derive(Clone, Debug)]
pub enum ActionType {
    SingleAction(SingleAction),
    MultiAction(MultiAction),
}

impl ActionType {
    fn get_action(&self) -> Rc<dyn Action> {
        match self {
            ActionType::SingleAction(action) => Rc::new(action.clone()),
            ActionType::MultiAction(action) => Rc::new(action.clone()),
        }
    }
}

impl CombatantConfig {
    pub fn to_combatant(&self) -> Combatant {
        let action_selection = crate::combat::action_selection::ActionSelection {
            actions: self.actions.iter().map(|x| x.get_action()).collect(),
        };
        Combatant::new_with_saves_and_resources(
            self.stats.max_hp,
            self.stats.ac,
            action_selection,
            self.stats.saves.clone(),
            self.resources.clone(),
        )
    }
}
