use std::collections::VecDeque;

use crate::{
    attack::damage::Damage,
    character::{character::Character, save::SaveModifiers},
};

use super::action::Action;

#[derive(Clone)]
pub struct CharacterWithActionQueue {
    character: Character,
    action_queue: ActionQueue,
}

impl CharacterWithActionQueue {
    pub fn new(character: Character, default_action: Action) -> Self {
        Self {
            character,
            action_queue: ActionQueue {
                default: default_action,
                next_actions: VecDeque::<Action>::new(),
            },
        }
    }
}

#[derive(Clone)]
pub enum Participant {
    Character(CharacterWithActionQueue),
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
            Participant::Character(character) => !character.character.is_dead(),
        }
    }

    fn take_damage(&mut self, damage: Damage) {
        match self {
            Participant::Character(character) => character.character.take_damage(damage),
        }
    }

    fn ac(&self) -> i16 {
        match self {
            Participant::Character(character) => character.character.ac(),
        }
    }

    fn saves(&self) -> &SaveModifiers {
        match self {
            Participant::Character(character) => character.character.saves(),
        }
    }
}

pub trait ActiveParticipant {
    fn take_action(&mut self) -> Action;
}

impl ActiveParticipant for Participant {
    fn take_action(&mut self) -> Action {
        match self {
            Participant::Character(character) => character.action_queue.pop(),
        }
    }
}

#[derive(Clone)]
pub struct ActionQueue {
    default: Action,
    next_actions: VecDeque<Action>,
}

impl ActionQueue {
    fn push(&mut self, action: Action) {
        self.next_actions.push_front(action);
    }

    fn pop(&mut self) -> Action {
        match self.next_actions.pop_back() {
            Some(action) => action,
            None => self.default.clone(),
        }
    }
}
