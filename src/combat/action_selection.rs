use crate::utils::{dice::Die, rollable::Rollable};

use super::action::Action;

pub struct ActionTable {}

pub trait UpdateReceiver {
    fn on_turn_start(&mut self);
    fn on_use(&mut self);
}

#[derive(Clone)]
pub struct Recharge {
    min_roll: u8,
    can_use: bool,
}

impl Recharge {
    fn new(min_roll: u8) -> Self {
        Self {
            min_roll,
            can_use: true,
        }
    }
}

#[derive(Clone)]
pub enum ActionState {
    Charges(u8),
    Recharge(Recharge),
}

impl ActionState {
    fn available(&self) -> bool {
        match self {
            ActionState::Charges(charges) => *charges > 0,
            ActionState::Recharge(recharge) => recharge.can_use,
        }
    }

    fn on_turn_start(&mut self) {
        match self {
            ActionState::Recharge(recharge) => {
                recharge.can_use = recharge.can_use || Die::D6.roll() > recharge.min_roll as u32
            }
            _ => (),
        }
    }

    fn on_use(&mut self) {
        match self {
            ActionState::Charges(charges) => *charges -= 1,
            ActionState::Recharge(recharge) => recharge.can_use = false,
        }
    }
}

#[derive(Clone)]
pub struct ActionSelection {
    default: Action,
    actions: Vec<StatefulAction>,
}

impl ActionSelection {
    pub fn new_default_only(default: Action) -> Self {
        Self::new(default, Vec::new())
    }

    pub fn new(default: Action, actions: Vec<StatefulAction>) -> Self {
        Self { default, actions }
    }

    pub fn update_and_select(&mut self) -> Action {
        // todo: improve action selection

        self.actions.iter_mut().for_each(|a| a.on_turn_start());
        self.actions
            .iter_mut()
            .find_map(|a| a.available().then(|| a.take()))
            .unwrap_or(self.default.clone())
    }
}

#[derive(Clone)]
pub struct StatefulAction {
    action: Action,
    state: ActionState,
}

impl StatefulAction {
    pub fn new_with_charges(action: Action, charges: u8) -> Self {
        Self {
            action,
            state: ActionState::Charges(charges),
        }
    }

    pub fn new_recharge(action: Action, min: u8) -> Self {
        Self {
            action,
            state: ActionState::Recharge(Recharge::new(min)),
        }
    }

    fn on_turn_start(&mut self) {
        self.state.on_turn_start()
    }

    fn available(&self) -> bool {
        self.state.available()
    }

    fn take(&mut self) -> Action {
        self.state.on_use();
        self.action.clone()
    }
}
