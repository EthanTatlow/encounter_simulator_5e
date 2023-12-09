use std::cmp::min;

use crate::{
    attack::damage::Damage,
    character::{character::Character, save::SaveModifiers},
};

use super::{action::Action, action_selection::ActionSelection};

pub struct Stats {
    hp: u32,
    ac: i16,
    saves: SaveModifiers,
}

pub struct Participant {
    stats: Stats,
    action_selection: ActionSelection,
}

impl Participant {
    pub fn new(hp: u32, ac: i16, saves: SaveModifiers, action_selection: ActionSelection) -> Self {
        Self {
            action_selection,
            stats: Stats { hp, ac, saves },
        }
    }

    pub fn new_from_character(character: Character, action_selection: ActionSelection) -> Self {
        Self {
            stats: Stats {
                hp: character.hp(),
                ac: character.ac(),
                saves: character.saves().clone(),
            },
            action_selection,
        }
    }

    pub fn hp(&self) -> u32 {
        self.stats.hp
    }
}

pub trait Damageable {
    fn is_conscious(&self) -> bool;
    fn take_damage(&mut self, damage: Damage);
    fn ac(&self) -> i16;
    fn saves(&self) -> &SaveModifiers;
}

impl Damageable for Participant {
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
}

pub trait ActiveParticipant {
    fn take_action(&mut self) -> Action;
}

impl ActiveParticipant for Participant {
    fn take_action(&mut self) -> Action {
        self.action_selection.update_and_select()
    }
}
