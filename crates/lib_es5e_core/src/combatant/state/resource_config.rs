use std::collections::HashMap;

use super::resources::Resource;

#[derive(Clone, Debug)]
pub struct ResourceConfig {
    pub(super) charges: u32,
    pub(super) max_charges: u32,
    pub(super) recharge: Option<Recharge>,
}

pub type Resources = HashMap<String, ResourceConfig>;

impl ResourceConfig {
    pub fn new(charges: u32, recharge: Option<Recharge>) -> Self {
        Self::new_with_max(charges, charges, recharge)
    }

    pub fn new_with_max(charges: u32, max_charges: u32, recharge: Option<Recharge>) -> Self {
        Self {
            charges,
            max_charges,
            recharge,
        }
    }

    pub(super) fn to_resource(&self, name: String) -> Resource {
        Resource {
            name,
            charges: self.charges,
            max_charges: self.max_charges,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Recharge {
    TurnStart, // recharge at start of turn
    Recharge5, // recharge at start of turn on rolling a 5 or 6 on a 1d6
    Recharge6, // recharge at start of turn on rolling a 6 on a d6
}
