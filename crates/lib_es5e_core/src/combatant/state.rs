use std::collections::HashMap;

use crate::{
    action::action::Action,
    utils::{dice::Die, rollable::Rollable},
};

#[derive(Clone, Debug)]
pub struct CombatantState {
    pub(crate) hp: u32,
    constant_resources: Vec<(String, u32)>,
    recharge5_resources: Vec<(String, u32)>,
    recharge6_resources: Vec<(String, u32)>,
    turn_recharge_resources: Vec<(String, u32)>,
}

pub type Resources = HashMap<String, Resource>;

#[derive(Clone, Debug)]
pub struct Resource {
    charges: u32,
    recharge: Option<Recharge>,
}

impl Resource {
    pub fn new(charges: u32, recharge: Option<Recharge>) -> Self {
        Self { charges, recharge }
    }
}

#[derive(Clone, Debug)]
pub enum Recharge {
    TurnStart, // recharge at start of turn
    Recharge5, // recharge at start of turn on rolling a 5 or 6 on a 1d6
    Recharge6, // recharge at start of turn on rolling a 6 on a d6
}

impl CombatantState {
    pub fn new(hp: u32, resources: Resources) -> Self {
        let mut constant_resources = Vec::new();
        let mut recharge5_resources = Vec::new();
        let mut recharge6_resources = Vec::new();
        let mut turn_recharge_resources = Vec::new();
        for (name, resource) in resources {
            match resource.recharge {
                None => &mut constant_resources,
                Some(Recharge::TurnStart) => &mut turn_recharge_resources, // recharge at start of turn
                Some(Recharge::Recharge5) => &mut recharge5_resources, // recharge at start of turn on rolling a 5 or 6 on a 1d6
                Some(Recharge::Recharge6) => &mut recharge6_resources, // recharge at start of turn on rolling a 6 on a d6
            }
            .push((name, resource.charges));
        }
        Self {
            hp,
            constant_resources,
            recharge5_resources,
            recharge6_resources,
            turn_recharge_resources,
        }
    }

    pub fn use_resource(&mut self, resource_cost: &HashMap<String, u32>) {
        resource_cost.iter().for_each(|(key, &val)| {
            self.find_resource_and_reduce(key, val);
        });
    }

    pub fn find_resource_and_reduce(&mut self, name: &str, cost: u32) {
        let mut resource_types = [
            &mut self.constant_resources,
            &mut self.recharge5_resources,
            &mut self.recharge6_resources,
            &mut self.turn_recharge_resources,
        ];
        resource_types.iter_mut().for_each(|resources| {
            if let Some((_, charges)) = resources.iter_mut().find(|(key, _val)| key == name) {
                *charges -= cost
            }
        });
    }

    pub fn find_and_reduce(stuff: &mut Vec<(String, u32)>, name: &str, cost: u32) {
        if let Some((_, charges)) = stuff.iter_mut().find(|(key, _val)| key == name) {
            *charges -= cost;
        }
    }

    pub fn recharge_on_turn_start(&mut self) {
        // TODO: skip unnecessary parts / rolls, e.g. by making more modular / adding logic to constructor
        let die_roll = Die::D6.roll();
        if die_roll >= 5 {
            add_charge(&mut self.recharge5_resources);
        }
        if die_roll >= 6 {
            add_charge(&mut self.recharge6_resources);
        }
        add_charge(&mut self.turn_recharge_resources);
    }

    pub fn can_execute(&self, x: &dyn Action) -> bool {
        let resource_types = [
            &self.constant_resources,
            &self.recharge5_resources,
            &self.recharge6_resources,
            &self.turn_recharge_resources,
        ];
        !x.resource_cost().iter().any(|(name, cost)| {
            resource_types.iter().any(|resources| {
                resources
                    .iter()
                    .any(|(key, available)| key == name && cost > available)
            })
        })
    }
}

fn add_charge(entries: &mut Vec<(String, u32)>) {
    entries.iter_mut().for_each(|(_, charges)| *charges += 1);
}
