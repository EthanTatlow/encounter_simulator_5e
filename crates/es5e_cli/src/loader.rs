use lib_es5e_core::utils::save::{Save, SaveType};
use lib_es5e_core::{action::attack::Attack, combatant::defences::save::SaveModifiers};
use lib_es5e_core::{
    action::negative_effect::negative_effect::NegativeEffect, attack::damage::DamageRoll,
};
use lib_es5e_core::{action::single::SingleAction, combat::action_selection::ActionSelection};
use lib_es5e_core::{
    action::{action, single::Execution},
    combatant::combatant::Combatant,
};
use lib_es5e_core::{
    action::{action::Action, multi::MultiAction},
    attack::save_based::SaveBasedAttack,
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{collections::HashMap, fs};
use std::{path::Path, rc::Rc};

pub fn load_combatants_from_file(file_path: &Path) -> Vec<Combatant> {
    let contents =
        fs::read_to_string(file_path).expect(format!("{file_path:?} not found").as_str());
    let values: Vec<CombatantConfig> = serde_yaml::from_str(contents.as_str())
        .expect(format!("Unable to parse {file_path:?}").as_str());
    let nr_combatants = values.len();
    println!("Combatants loaded from {file_path:?}: {nr_combatants}");

    values.into_iter().map(|e| e.into()).collect()
}

#[derive(Debug, Serialize, Deserialize)]
struct CombatantConfig {
    pub name: String,
    pub hp: u32,
    pub ac: i16,
    pub saves: SavesConfig,
    pub actions: ActionSelectionConfig,
}

impl From<CombatantConfig> for Combatant {
    fn from(enemy: CombatantConfig) -> Self {
        Self::new(
            enemy.hp,
            enemy.ac,
            SaveModifiers::from(enemy.saves),
            ActionSelection::from(enemy.actions),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavesConfig {
    pub str: i16,
    pub dex: i16,
    pub con: i16,
    pub int: i16,
    pub wis: i16,
    pub cha: i16,
}

impl From<SavesConfig> for SaveModifiers {
    fn from(saves: SavesConfig) -> Self {
        SaveModifiers::new(
            saves.str, saves.dex, saves.con, saves.int, saves.wis, saves.cha,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionSelectionConfig {
    pub default: Vec<ActionConfig>,
    pub special: Vec<StatefulActionConfig>,
}

fn multiple_actions_to(actions: Vec<ActionConfig>) -> Rc<dyn Action> {
    Rc::new(MultiAction::new(
        actions.into_iter().map(|x| x.into()).collect(),
    ))
}

impl From<ActionSelectionConfig> for ActionSelection {
    fn from(actions: ActionSelectionConfig) -> Self {
        let default_multi = multiple_actions_to(actions.default);
        let mut actions: Vec<_> = actions
            .special
            .into_iter()
            .map(|x| multiple_actions_to(x.actions))
            .collect();
        actions.push(default_multi);
        ActionSelection { actions }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatefulActionConfig {
    actions: Vec<ActionConfig>,
    recharge: u8,
    // resource: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionConfig {
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

impl From<ActionConfig> for SingleAction {
    fn from(val: ActionConfig) -> Self {
        Self {
            execution: match val {
                ActionConfig::SaveBasedAttack {
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
                ActionConfig::Attack { name: _, atk, dmg } => Execution::Attack(Attack::new(
                    atk,
                    DamageRoll::from_str(dmg.as_str()).unwrap(),
                )),
            },
            resource_cost: HashMap::new(), // TODO
        }
    }
}

#[cfg(test)]
mod test {
    use lib_es5e_core::combatant::combatant::Combatant;

    use crate::loader::CombatantConfig;

    // Note: the API is currently very volatile, so more detailed tests are omitted for the time being
    #[test]
    fn test_parse() {
        let yaml = "
  - name: dragon
    hp: 367
    ac: 22
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

        let combatants: Vec<CombatantConfig> =
            serde_yaml::from_str(yaml).expect("unable to parse test data");
        let part: Vec<Combatant> = combatants.into_iter().map(|e| e.into()).collect();
        assert_eq!(part.len(), 1);
    }
}
