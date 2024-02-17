#[cfg(test)]
use mockall::automock;
use std::cmp::min;

use crate::{attack::damage::Damage, character::save::SaveModifiers};

use super::{action::Action, action_selection::ActionSelection};

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

    pub fn hp(&self) -> u32 {
        self.stats.hp
    }
}

#[cfg_attr(test, automock)]
pub trait Target {
    fn is_conscious(&self) -> bool;
    fn take_damage(&mut self, damage: Damage);
    fn ac(&self) -> i16;
    fn hp(&self) -> u32;
    fn saves(&self) -> &SaveModifiers;
}

impl Target for Combatant {
    fn is_conscious(&self) -> bool {
        self.stats.hp > 0
    }

    fn take_damage(&mut self, damage: Damage) {
        self.stats.hp -= min(damage.amount(), self.stats.hp)
    }

    fn ac(&self) -> i16 {
        self.stats.ac
    }

    fn saves(&self) -> &SaveModifiers {
        &self.stats.saves
    }

    fn hp(&self) -> u32 {
        self.stats.hp
    }
}

pub trait ActiveCombatant {
    fn take_action(&mut self) -> Action;
}

impl ActiveCombatant for Combatant {
    fn take_action(&mut self) -> Action {
        self.action_selection.update_and_select()
    }
}
