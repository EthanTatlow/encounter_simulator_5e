use crate::utils::save::SaveType;

use super::ability::AbilityModifiers;

#[derive(Clone, Debug)]
pub struct SaveModifiers {
    modifiers: AbilityModifiers,
}

impl SaveModifiers {
    pub fn new(str: i16, dex: i16, con: i16, int: i16, wis: i16, cha: i16) -> Self {
        Self {
            modifiers: AbilityModifiers::new(str, dex, con, int, wis, cha),
        }
    }

    pub fn from_modifiers(modifiers: AbilityModifiers) -> Self {
        return Self { modifiers };
    }

    pub fn default() -> Self {
        Self {
            modifiers: AbilityModifiers::default(),
        }
    }

    pub fn modifier(&self, save_type: &SaveType) -> i16 {
        match save_type {
            SaveType::STR => self.modifiers.str(),
            SaveType::DEX => self.modifiers.dex(),
            SaveType::CON => self.modifiers.con(),
            SaveType::INT => self.modifiers.int(),
            SaveType::WIS => self.modifiers.wis(),
            SaveType::CHA => self.modifiers.cha(),
        }
    }
}
