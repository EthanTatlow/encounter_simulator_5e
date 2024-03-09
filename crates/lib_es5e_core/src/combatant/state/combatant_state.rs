use std::collections::BTreeMap;

use crate::{
    action::action::Action,
    utils::{dice::Die, rollable::Rollable},
};

use super::{
    resource_config::{Recharge, ResourceKey},
    resources::{reset_charge_to_max, Resources},
    ResourceCfgs,
};

pub type ResourceCosts = BTreeMap<usize, u32>;

#[derive(Clone, Debug)]
pub struct CombatantState {
    pub(crate) hp: u32,
    constant_resources: Resources,
    recharge5_resources: Resources,
    recharge6_resources: Resources,
    turn_recharge_resources: Resources,
}

impl CombatantState {
    pub fn new(hp: u32, resources: ResourceCfgs) -> Self {
        let mut constant_resources = Resources::new();
        let mut recharge5_resources = Resources::new();
        let mut recharge6_resources = Resources::new();
        let mut turn_recharge_resources = Resources::new();
        for (name, res_cfg) in resources {
            match res_cfg.recharge {
                None => &mut constant_resources,
                Some(Recharge::TurnStart) => &mut turn_recharge_resources, // recharge at start of turn
                Some(Recharge::Recharge5) => &mut recharge5_resources, // recharge at start of turn on rolling a 5 or 6 on a 1d6
                Some(Recharge::Recharge6) => &mut recharge6_resources, // recharge at start of turn on rolling a 6 on a d6
            }
            .insert(name, res_cfg.into());
        }

        Self {
            hp,
            constant_resources,
            recharge5_resources,
            recharge6_resources,
            turn_recharge_resources,
        }
    }

    pub fn use_resource(&mut self, resource_cost: &ResourceCosts) {
        resource_cost.iter().for_each(|(&key, &val)| {
            self.find_resource_and_reduce(key, val);
        });
    }

    fn find_resource_and_reduce(&mut self, key: ResourceKey, cost: u32) {
        let mut resource_types = [
            &mut self.constant_resources,
            &mut self.recharge5_resources,
            &mut self.recharge6_resources,
            &mut self.turn_recharge_resources,
        ];
        resource_types.iter_mut().any(|resources| {
            if let Some(resource) = resources.get_mut(&key) {
                if resource.charges >= cost {
                    resource.charges -= cost;
                } else {
                    eprintln!("Tried to use more charges than available for resource {key}");
                }
                true
            } else {
                false
            }
        });
    }

    pub fn recharge_on_turn_start(&mut self) {
        // TODO: skip unnecessary parts / rolls, e.g. by making more modular / adding logic to constructor
        let die_roll = Die::D6.roll();
        if die_roll >= 5 {
            reset_charge_to_max(&mut self.recharge5_resources);
        }
        if die_roll >= 6 {
            reset_charge_to_max(&mut self.recharge6_resources);
        }
        reset_charge_to_max(&mut self.turn_recharge_resources);
    }

    pub fn can_execute(&self, x: &dyn Action) -> bool {
        let resource_types = [
            &self.constant_resources,
            &self.recharge5_resources,
            &self.recharge6_resources,
            &self.turn_recharge_resources,
        ];
        !x.resource_costs().iter().any(|(key, &cost)| {
            resource_types.iter().any(|resources| {
                resources
                    .get(key)
                    .is_some_and(|resource| cost > resource.charges)
            })
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::combat::encounter::IntMutCombatant;
    use crate::combatant::state::ResourceCfg;
    use mockall::mock;

    #[test]
    fn test_use_resource() {
        let mut resource_cfgs = ResourceCfgs::new();

        resource_cfgs.insert(0, ResourceCfg::new(5, None));
        let mut combatant = CombatantState::new(10, resource_cfgs);

        let mut resource_cost = ResourceCosts::new();
        resource_cost.insert(0, 3);

        let mut mock_action = MockAction::new();
        mock_action
            .expect_resource_costs()
            .return_const(resource_cost.clone());

        assert!(combatant.can_execute(&mock_action));
        combatant.use_resource(&resource_cost);
        assert!(!combatant.can_execute(&mock_action));
    }

    #[test]
    fn test_recharge_on_turn_start() {
        let mut resource_cfgs = ResourceCfgs::new();

        resource_cfgs.insert(1, ResourceCfg::new(1, Some(Recharge::TurnStart)));
        let mut combatant = CombatantState::new(10, resource_cfgs);

        let mut resource_cost = ResourceCosts::new();
        resource_cost.insert(1, 1);

        let mut mock_action = MockAction::new();
        mock_action
            .expect_resource_costs()
            .return_const(resource_cost.clone());

        combatant.use_resource(&resource_cost);
        assert!(!combatant.can_execute(&mock_action));
        combatant.recharge_on_turn_start();
        assert!(combatant.can_execute(&mock_action));
    }

    #[test]
    fn test_recharge_on_turn_without_overcharging() {
        let mut resource_cfgs = ResourceCfgs::new();

        resource_cfgs.insert(1, ResourceCfg::new(1, Some(Recharge::TurnStart)));
        let mut combatant = CombatantState::new(10, resource_cfgs);

        let mut resource_cost = ResourceCosts::new();
        resource_cost.insert(1, 1);

        let mut mock_action = MockAction::new();
        mock_action
            .expect_resource_costs()
            .return_const(resource_cost.clone());

        combatant.recharge_on_turn_start();
        assert!(combatant.can_execute(&mock_action));
        combatant.use_resource(&resource_cost);
        assert!(!combatant.can_execute(&mock_action));
    }

    #[test]
    fn test_using_unavailable_resource() {
        let mut resource_cfgs = ResourceCfgs::new();

        resource_cfgs.insert(1, ResourceCfg::new(1, None));
        let mut combatant = CombatantState::new(10, resource_cfgs);

        let mut resource_cost = ResourceCosts::new();
        resource_cost.insert(1, 2);

        let mut mock_action = MockAction::new();
        mock_action
            .expect_resource_costs()
            .return_const(resource_cost.clone());

        assert!(!combatant.can_execute(&mock_action));

        combatant.use_resource(&resource_cost);
    }

    mock! {
        Action {}
        impl Action for Action {
            fn execute(&self, _allies: &[IntMutCombatant], enemies: &[IntMutCombatant]);
            fn resource_costs(&self) -> &ResourceCosts;
        }
        impl Clone for Action {
            fn clone(&self) -> Self;
        }
    }
}
