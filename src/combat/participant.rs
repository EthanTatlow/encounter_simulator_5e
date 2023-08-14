use std::collections::VecDeque;

use crate::{
    attack::damage::Damage,
    character::{character::Character, save::SaveModifiers},
};

use super::{action::Action, action_selection::ActionSelection};

#[derive(Clone)]
pub struct CharacterWithActionQueue {
    character: Character,
    action_queue: ActionSelection,
}

#[derive(Clone)]
pub enum Participant {
    Character(Character, ActionSelection),
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
        }
    }

    fn take_damage(&mut self, damage: Damage) {
        match self {
            Participant::Character(character, _) => character.take_damage(damage),
        }
    }

    fn ac(&self) -> i16 {
        match self {
            Participant::Character(character, _) => character.ac(),
        }
    }

    fn saves(&self) -> &SaveModifiers {
        match self {
            Participant::Character(character, _) => character.saves(),
        }
    }
}

pub trait ActiveParticipant {
    fn take_action(&mut self) -> Action;
}

impl ActiveParticipant for Participant {
    fn take_action(&mut self) -> Action {
        match self {
            Participant::Character(_, actions) => actions.update_and_select(),
        }
    }
}
