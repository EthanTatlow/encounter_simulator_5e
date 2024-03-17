use std::str::FromStr;

use lib_es5e_core::{
    action::{
        attack::Attack,
        multi::MultiAction,
        negative_effect::negative_effect::NegativeEffect,
        single::{Execution, SingleAction},
    },
    attack::{damage::DamageRoll, save_based::SaveBasedAttack},
    combatant::{
        config::ActionType,
        state::{Recharge, ResourceCfg, ResourceCfgs, ResourceCosts},
    },
    utils::save::{Save, SaveType},
};
use serde::{Deserialize, Serialize};

use crate::rules::{CharacterLvl, Class, SpellLvl};

const UNIQUE_ACTION_COST_ID_OFFSET: usize = 2000;
fn to_unique_action_cost_id(idx: usize) -> usize {
    idx + UNIQUE_ACTION_COST_ID_OFFSET
}

pub fn get_action_selection_and_resources(
    action_dtos: Vec<ActionDto>,
    class_level: Option<(Class, CharacterLvl)>,
) -> (Vec<ActionType>, ResourceCfgs) {
    let (actions, mut resource_cfg) = to_actions_and_resource_cfg(action_dtos);
    if let Some((class, lvl)) = class_level {
        let spell_slots = class.spell_slots_for_char_lvl(lvl);
        spell_slots.into_iter().for_each(|(spell_lvl, charges)| {
            resource_cfg.insert(
                spell_lvl_to_resource_key(spell_lvl),
                ResourceCfg::new(charges, None),
            );
        });
    }
    (actions, resource_cfg)
}

fn into_multi_action(action_dtos: Vec<SingleActionDto>) -> (ActionType, ResourceCfgs) {
    let (actions, resource_cfg) = action_dtos
        .into_iter()
        .enumerate()
        .map(|(idx, action)| map_single_action(idx, action))
        .map(|(a, b)| (vec![a], b))
        .reduce(|(mut a, mut b), (x, y)| {
            a.extend(x);
            b.extend(y);
            (a, b)
        })
        .unwrap_or_default();
    let action = ActionType::MultiAction(MultiAction::new(actions));
    (action, resource_cfg)
}

fn to_actions_and_resource_cfg(action_dtos: Vec<ActionDto>) -> (Vec<ActionType>, ResourceCfgs) {
    let (actions, resource_cfg) = action_dtos
        .into_iter()
        .enumerate()
        .map(|(idx, action)| match action {
            ActionDto::SingleAction(action) => {
                let (action, resource_cfg) = map_single_action(idx, action);
                (ActionType::SingleAction(action), resource_cfg)
            }
            ActionDto::MultiAction(actions) => into_multi_action(actions),
        })
        .map(|(a, b)| (vec![a], b))
        .reduce(|(mut a, mut b), (x, y)| {
            a.extend(x);
            b.extend(y);
            (a, b)
        })
        .unwrap_or_default();
    (actions, resource_cfg)
}

fn map_single_action(
    idx: usize,
    action: SingleActionDto,
) -> (SingleAction, std::collections::BTreeMap<usize, ResourceCfg>) {
    let (partial_resource_cfg, resource_cost) =
        resource_costs_to_cfg_and_cost(idx, action.resource_costs.unwrap_or_default());
    let execution = action.attack.into();
    let action = SingleAction {
        resource_cost,
        execution,
    };
    (action, partial_resource_cfg)
}

fn resource_costs_to_cfg_and_cost(
    idx: usize,
    resource_costs: Vec<ResourceCostDto>,
) -> (ResourceCfgs, ResourceCosts) {
    let mut partial_resource_cfg = ResourceCfgs::new();
    let mut resource_cost = ResourceCosts::new();
    resource_costs.into_iter().for_each(|cost| match cost {
        ResourceCostDto::ById { id, amount } => {
            resource_cost.insert(id, amount);
        }
        ResourceCostDto::Unique { available: amount } => {
            resource_cost.insert(to_unique_action_cost_id(idx), 1);
            partial_resource_cfg.insert(
                to_unique_action_cost_id(idx),
                ResourceCfg::new(amount, None),
            );
        }
        ResourceCostDto::SpellSlot(spell_lvl) => {
            resource_cost.insert(spell_lvl_to_resource_key(spell_lvl), 1);
        }
        ResourceCostDto::Recharge5 => {
            resource_cost.insert(to_unique_action_cost_id(idx), 1);
            partial_resource_cfg.insert(
                to_unique_action_cost_id(idx),
                ResourceCfg::new(1, Some(Recharge::Recharge5)),
            );
        }
    });
    (partial_resource_cfg, resource_cost)
}

const BASE_SPELL_SLOT_KEY: usize = 1000;
pub const fn spell_lvl_to_resource_key(spell_lvl: SpellLvl) -> usize {
    BASE_SPELL_SLOT_KEY
        + match spell_lvl {
            SpellLvl::Lvl1 => 1,
            SpellLvl::Lvl2 => 2,
            SpellLvl::Lvl3 => 3,
            SpellLvl::Lvl4 => 4,
            SpellLvl::Lvl5 => 5,
            SpellLvl::Lvl6 => 6,
            SpellLvl::Lvl7 => 7,
            SpellLvl::Lvl8 => 8,
            SpellLvl::Lvl9 => 9,
        }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RechargeActionDto {
    actions: Vec<ActionDto>,
    recharge: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionDto {
    SingleAction(SingleActionDto),
    MultiAction(Vec<SingleActionDto>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleActionDto {
    attack: AttackDto,
    resource_costs: Option<Vec<ResourceCostDto>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AttackDto {
    Attack {
        name: String,
        atk: i16,
        dmg: String,
    },
    SaveBasedAttack {
        name: String,
        save_dc: i16,
        save_type: SaveType,
        targets: usize,
        damage: String,
        half_on_success: bool,
    },
}

mod action_dto {

    use lib_es5e_core::utils::save::SaveType;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Attack {
        name: String,
        atk: i16,
        dmg: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SaveBasedAttack {
        name: String,
        save_dc: i16,
        save_type: SaveType,
        targets: usize,
        damage: String,
        half_on_success: bool,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceCostDto {
    ById { id: usize, amount: u32 },
    Unique { available: u32 },
    Recharge5,
    SpellSlot(SpellLvl),
}

impl From<AttackDto> for Execution {
    fn from(attack_dto: AttackDto) -> Self {
        match attack_dto {
            AttackDto::SaveBasedAttack {
                name: _,
                save_dc,
                save_type,
                targets,
                damage,
                half_on_success,
            } => Execution::ApplyNegativeEffect(NegativeEffect::Saveable(SaveBasedAttack::new(
                Save::new(save_type, save_dc),
                targets,
                half_on_success,
                DamageRoll::from_str(damage.as_str()).unwrap(),
            ))),
            AttackDto::Attack { name: _, atk, dmg } => Execution::Attack(Attack::new(
                atk,
                DamageRoll::from_str(dmg.as_str()).unwrap(),
            )),
        }
    }
}
