use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct CombatantState {
    pub(crate) hp: u32,
    resources: HashMap<String, Resource>,
}

#[derive(Clone, Debug)]
struct Resource {
    charges: u32,
    recharge: Option<Recharge>,
}

#[derive(Clone, Debug)]
enum Recharge {
    TurnStart, // recharge at start of turn
    Recharge5, // recharge at start of turn on rolling a 5 or 6 on a 1d6
    Recharge6, // recharge at start of turn on rolling a 6 on a d6
}

impl CombatantState {
    pub fn new(hp: u32) -> Self {
        Self {
            hp,
            resources: HashMap::new(),
        }
    }
}
