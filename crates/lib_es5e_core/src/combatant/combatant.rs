use std::{cmp::min, rc::Rc};

use crate::{
    action::action::Action, attack::damage::Damage, combat::action_selection::ActionSelection,
    combatant::defences::save::SaveModifiers,
};

use super::{
    state::{CombatantState, Resources},
    stats::CombatantStats,
};

#[derive(Clone, Debug)]
pub struct Combatant {
    pub stats: CombatantStats,
    state: CombatantState,
    action_selection: ActionSelection,
}

impl Combatant {
    pub fn new_with_saves_and_resources(
        max_hp: u32,
        ac: i16,
        initiative: i16,
        action_selection: ActionSelection,
        saves: SaveModifiers,
        resources: Resources,
    ) -> Self {
        Self {
            action_selection,
            stats: CombatantStats {
                max_hp,
                ac,
                saves,
                initiative,
            },
            state: CombatantState::new(max_hp, resources),
        }
    }

    pub fn new(
        max_hp: u32,
        ac: i16,
        init: i16,
        saves: SaveModifiers,
        action_selection: ActionSelection,
    ) -> Self {
        Self::new_with_saves_and_resources(max_hp, ac, init, action_selection, saves, Resources::new())
    }

    pub fn first_available_action(&self) -> Option<Rc<dyn Action>> {
        self.action_selection
            .actions
            .iter()
            .find(|x| self.state.can_execute(x.as_ref()))
            .cloned()
    }

    pub fn update_resources_on_start(&mut self) {
        self.state.recharge_on_turn_start();
    }

    pub fn use_resources(&mut self, action: &dyn Action) {
        self.state.use_resource(action.resource_cost())
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
