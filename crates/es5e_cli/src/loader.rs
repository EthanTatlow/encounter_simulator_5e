use lib_es5e_core::{
    action::attack::Attack,
    combatant::{
        config::CombatantConfig,
        defences::save::SaveModifiers,
        state::{Recharge, ResourceCfg, ResourceCosts},
    },
};
use lib_es5e_core::{action::multi::MultiAction, attack::save_based::SaveBasedAttack};
use lib_es5e_core::{
    action::negative_effect::negative_effect::NegativeEffect, attack::damage::DamageRoll,
};
use lib_es5e_core::{action::single::Execution, combatant::config::ActionType};
use lib_es5e_core::{action::single::SingleAction, combatant::stats::CombatantStats};
use lib_es5e_core::{
    combatant::state::ResourceCfgs,
    utils::save::{Save, SaveType},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub fn load_combatants_from_file(file_path: &Path) -> Vec<CombatantConfig> {
    let contents =
        fs::read_to_string(file_path).expect(format!("{file_path:?} not found").as_str());
    let values: Vec<CombatantDto> = serde_yaml::from_str(contents.as_str())
        .expect(format!("Unable to parse {file_path:?}").as_str());
    let nr_combatants = values.len();
    println!("Combatants loaded from {file_path:?}: {nr_combatants}");

    values.into_iter().map(|e| e.into()).collect()
}

#[derive(Debug, Serialize, Deserialize)]
struct CombatantDto {
    pub name: String,
    pub hp: u32,
    pub ac: i16,
    pub init: i16,
    pub saves: SaveModifiersDto,
    pub actions: ActionSelectionDto,
}

impl From<CombatantDto> for CombatantConfig {
    fn from(dto: CombatantDto) -> Self {
        let (actions, resources) = get_action_selection_and_resources(dto.actions);
        Self {
            resources,
            actions,
            stats: CombatantStats {
                max_hp: dto.hp,
                ac: dto.ac,
                initiative: dto.init,
                saves: dto.saves.into(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveModifiersDto {
    pub str: i16,
    pub dex: i16,
    pub con: i16,
    pub int: i16,
    pub wis: i16,
    pub cha: i16,
}

impl From<SaveModifiersDto> for SaveModifiers {
    fn from(saves: SaveModifiersDto) -> Self {
        SaveModifiers::new(
            saves.str, saves.dex, saves.con, saves.int, saves.wis, saves.cha,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionSelectionDto {
    pub default: Vec<ActionDto>,
    pub special: Vec<RechargeActionDto>,
}

fn into_multi_action(actions: Vec<ActionDto>) -> ActionType {
    ActionType::MultiAction(MultiAction::new(
        actions.into_iter().map(|x| x.into()).collect(),
    ))
}

fn multiple_actions_with_cost(actions: Vec<ActionDto>, name: usize) -> ActionType {
    let mut resource_cost = ResourceCosts::new();
    resource_cost.insert(name, 1);
    let mut actions: Vec<SingleAction> = actions.into_iter().map(|x| x.into()).collect();
    if !actions.is_empty() {
        let action = actions.remove(0);
        actions.insert(
            0,
            SingleAction {
                resource_cost,
                execution: action.execution.clone(),
            },
        );
    };
    ActionType::MultiAction(MultiAction::new(actions))
}

fn get_action_selection_and_resources(
    actions: ActionSelectionDto,
) -> (Vec<ActionType>, ResourceCfgs) {
    let default_multi = into_multi_action(actions.default);
    let resources: ResourceCfgs = actions
        .special
        .iter()
        .enumerate()
        .map(|(i, a)| {
            (
                i,
                ResourceCfg::new(
                    1,
                    match a.recharge {
                        5 => Some(Recharge::Recharge5),
                        6 => Some(Recharge::Recharge6),
                        0 => Some(Recharge::TurnStart),
                        _ => None,
                    },
                ),
            )
        })
        .collect();
    let mut actions: Vec<_> = actions
        .special
        .into_iter()
        .enumerate()
        .map(|(i, conf)| multiple_actions_with_cost(conf.actions, i))
        .collect();
    actions.push(default_multi);
    (actions, resources)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RechargeActionDto {
    actions: Vec<ActionDto>,
    recharge: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionDto {
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

impl From<ActionDto> for SingleAction {
    fn from(val: ActionDto) -> Self {
        Self {
            execution: match val {
                ActionDto::SaveBasedAttack {
                    name: _,
                    save_dc,
                    save_type,
                    targets,
                    damage,
                    half_on_success,
                } => {
                    Execution::ApplyNegativeEffect(NegativeEffect::Saveable(SaveBasedAttack::new(
                        Save::new(save_type, save_dc),
                        targets,
                        half_on_success,
                        DamageRoll::from_str(damage.as_str()).unwrap(),
                    )))
                }
                ActionDto::Attack { name: _, atk, dmg } => Execution::Attack(Attack::new(
                    atk,
                    DamageRoll::from_str(dmg.as_str()).unwrap(),
                )),
            },
            resource_cost: ResourceCosts::new(), // TODO
        }
    }
}

#[cfg(test)]
mod test {
    use lib_es5e_core::combatant::config::CombatantConfig;

    use crate::loader::CombatantDto;

    // Note: the API is currently very volatile, so more detailed tests are omitted for the time being
    #[test]
    fn test_parse() {
        let yaml = "
  - name: dragon
    hp: 367
    ac: 22
    init: 1
    saves:
      str: 8
      dex: 9
      con: 14
      int: 3
      wis: 9
      cha: 11
    actions:
      default:
        - &claws !Attack
          name: claws
          atk: 15
          dmg: 2d10+8
        - *claws
        - !Attack
          name: bite
          atk: 15
          dmg: 2d6+8
      special:
        - recharge: 5 # recharges on a 5 or higher when rolling 1d6
          actions:
            - !SaveBasedAttack
              name: breath weapon
              save_dc: 22
              save_type: !DEX
              targets: 3
              damage: 15d8
              half_on_success: true
    ";

        let combatants: Vec<CombatantDto> =
            serde_yaml::from_str(yaml).expect("unable to parse test data");
        let part: Vec<CombatantConfig> = combatants.into_iter().map(|e| e.into()).collect();
        assert_eq!(part.len(), 1);
    }
}
