use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::attack::attack::Attack;
use crate::attack::damage::DamageRoll;
use crate::attack::save_based;
use crate::character::save::SaveModifiers;
use crate::combat::action;
use crate::combat::action_selection::{ActionSelection, StatefulAction};
use crate::combat::participant::Participant;
use crate::utils::save::{Save, SaveType};
use serde::{Deserialize, Serialize};

pub fn load_participants_from_file(file_path: &Path) -> Vec<Participant> {
    let contents =
        fs::read_to_string(file_path).expect(format!("{file_path:?} not found").as_str());
    let values: Vec<ParticipantConfig> = serde_yaml::from_str(contents.as_str())
        .expect(format!("Unable to parse {file_path:?}").as_str());
    println!("Participants loaded:\n{values:#?}\n");

    values.into_iter().map(|e| e.into()).collect()
}

#[derive(Debug, Serialize, Deserialize)]
struct ParticipantConfig {
    pub name: String,
    pub hp: u32,
    pub ac: i16,
    pub saves: SavesConfig,
    pub actions: ActionSelectionConfig,
}

impl From<ParticipantConfig> for Participant {
    fn from(enemy: ParticipantConfig) -> Self {
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
    pub default: ActionConfig,
    pub special: Vec<StatefulActionConfig>,
}

impl From<ActionSelectionConfig> for ActionSelection {
    fn from(actions: ActionSelectionConfig) -> Self {
        ActionSelection::new(
            action::Action::from(actions.default),
            actions
                .special
                .into_iter()
                .map(|a| StatefulAction::from(a))
                .collect(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatefulActionConfig {
    action: ActionConfig,
    recharge: u8,
}

impl From<StatefulActionConfig> for StatefulAction {
    fn from(stateful_action: StatefulActionConfig) -> Self {
        StatefulAction::new_recharge(
            action::Action::from(stateful_action.action),
            stateful_action.recharge,
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionConfig {
    MultiAction(Vec<ActionConfig>),
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

impl From<ActionConfig> for action::Action {
    fn from(val: ActionConfig) -> Self {
        match val {
            ActionConfig::SaveBasedAttack {
                name: _,
                save_dc,
                save_type,
                targets,
                damage,
                half_on_success,
            } => action::Action::SaveBasedAttack(save_based::SaveBasedAttack::new(
                Save::new(save_type, save_dc),
                targets,
                half_on_success,
                DamageRoll::from_str(damage.as_str()).unwrap(),
            )),
            ActionConfig::MultiAction(actions) => action::Action::MultiAction(
                actions
                    .into_iter()
                    .map(|x| action::Action::from(x))
                    .collect(),
            ),
            ActionConfig::Attack { name: _, atk, dmg } => action::Action::SingleAttack(
                Attack::new(atk, DamageRoll::from_str(dmg.as_str()).unwrap()),
            ),
        }
    }
}

#[test]
fn test_stuff() {
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
        !MultiAction
        # - 
          # Currently missing status effects
          # - name: frightening presence
          #   type: save_based_attack
          #   targets: 100
          #   save_dc: 19
          #   save_type: !WIS
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
          action:
            !SaveBasedAttack
            name: breath weapon
            save_dc: 22
            save_type: !DEX
            targets: 3
            damage: 15d8
            half_on_success: true

    ";

    let values: Vec<ParticipantConfig> = serde_yaml::from_str(yaml).unwrap();
    let part: Vec<Participant> = values.into_iter().map(|e| e.into()).collect();
    print!("{part:?}");
}
