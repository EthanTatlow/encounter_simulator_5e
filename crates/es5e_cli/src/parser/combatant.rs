use lib_es5e_core::combatant::{config::CombatantConfig, stats::CombatantStats};
use serde::{Deserialize, Serialize};

use crate::rules::{CharacterLvl, Class};

use super::actions::{get_action_selection_and_resources, ActionDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct CombatantDto {
    pub name: String,
    pub hp: u32,
    pub ac: i16,
    pub init: i16,
    // Note: currently only used for spell slots
    pub class_lvl: Option<(Class, CharacterLvl)>,
    pub saves: SaveModifiersDto,
    pub actions: Vec<ActionDto>,
}

impl From<CombatantDto> for CombatantConfig {
    fn from(dto: CombatantDto) -> Self {
        let (actions, resources) = get_action_selection_and_resources(dto.actions, dto.class_lvl);
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
