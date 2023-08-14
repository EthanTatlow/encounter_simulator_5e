use std::cmp::min;

use crate::{
    attack::damage::Damage,
    character::{character::Character, save::SaveModifiers},
};

use super::{action::Action, action_selection::ActionSelection};

pub struct SimpleStats {
    hp: u32,
    ac: i16,
    saves: SaveModifiers,
}

pub enum Participant {
    Character(Character, ActionSelection),
    Simple(SimpleStats, ActionSelection),
}

impl Participant {
    pub fn new_simple(
        hp: u32,
        ac: i16,
        saves: SaveModifiers,
        action_selection: ActionSelection,
    ) -> Self {
        return Self::Simple(SimpleStats { hp, ac, saves }, action_selection);
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
        match self {
            Participant::Character(character, _) => !character.is_dead(),
            Participant::Simple(stats, _) => stats.hp > 0,
        }
    }

    fn take_damage(&mut self, damage: Damage) {
        match self {
            Participant::Character(character, _) => character.take_damage(damage),
            Participant::Simple(stats, _) => stats.hp -= min(damage.amount(), stats.hp),
        }
    }

    fn ac(&self) -> i16 {
        match self {
            Participant::Character(character, _) => character.ac(),
            Participant::Simple(stats, _) => stats.ac,
        }
    }

    fn saves(&self) -> &SaveModifiers {
        match self {
            Participant::Character(character, _) => character.saves(),
            Participant::Simple(stats, _) => &stats.saves,
        }
    }
}

pub trait ActiveParticipant {
    fn take_action(&mut self) -> Action;
}

impl ActiveParticipant for Participant {
    fn take_action(&mut self) -> Action {
        match self {
            Participant::Character(_, actions) | Participant::Simple(_, actions) => {
                actions.update_and_select()
            }
        }
    }
}
