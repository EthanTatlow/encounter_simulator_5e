use std::{cmp::min, rc::Rc};

use crate::{
    action::action::Action, attack::damage::Damage, combat::action_selection::ActionSelection,
    combatant::defences::save::SaveModifiers,
};

use super::{state::CombatantState, stats::CombatantStats};

#[derive(Clone, Debug)]
pub struct Combatant {
    pub stats: CombatantStats,
    state: CombatantState,
    action_selection: ActionSelection,
}

impl Combatant {
    pub fn new(
        max_hp: u32,
        ac: i16,
        saves: SaveModifiers,
        action_selection: ActionSelection,
    ) -> Self {
        Self {
            action_selection,
            stats: CombatantStats {
                max_hp,
                ac,
                saves,
                initiative: 0, // TODO
            },
            state: CombatantState::new(max_hp),
        }
    }

    pub fn first_available_action(&self) -> Rc<dyn Action> {
        self.action_selection
            .actions
            .iter()
            .find_map(|x| Some(x.clone())) // TODO: filter
            .unwrap()
    }

    pub fn is_conscious(&self) -> bool {
        self.state.hp > 0
    }

    pub fn take_damage(&mut self, damage: Damage) {
        self.state.hp -= min(damage.amount(), self.state.hp)
    }

    pub fn ac(&self) -> i16 {
        self.stats.ac
    }

    pub fn saves(&self) -> &SaveModifiers {
        &self.stats.saves
    }

    pub fn hp(&self) -> u32 {
        self.state.hp
    }
}
