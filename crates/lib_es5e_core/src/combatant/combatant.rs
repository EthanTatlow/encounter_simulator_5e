use std::cmp::min;

use crate::{
    action::action::Action, attack::damage::Damage, combat::action_selection::ActionSelection,
    combatant::defences::save::SaveModifiers,
};

#[derive(Clone, Debug)]
pub struct Stats {
    hp: u32,
    ac: i16,
    saves: SaveModifiers,
}

#[derive(Clone, Debug)]
pub struct Combatant {
    stats: Stats,
    action_selection: ActionSelection,
}

impl Combatant {
    pub fn new(hp: u32, ac: i16, saves: SaveModifiers, action_selection: ActionSelection) -> Self {
        Self {
            action_selection,
            stats: Stats { hp, ac, saves },
        }
    }

    pub fn take_action(&mut self) -> Action {
        self.action_selection.update_and_select()
    }

    pub fn is_conscious(&self) -> bool {
        self.stats.hp > 0
    }

    pub fn take_damage(&mut self, damage: Damage) {
        self.stats.hp -= min(damage.amount(), self.stats.hp)
    }

    pub fn ac(&self) -> i16 {
        self.stats.ac
    }

    pub fn saves(&self) -> &SaveModifiers {
        &self.stats.saves
    }

    pub fn hp(&self) -> u32 {
        self.stats.hp
    }
}
