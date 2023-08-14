use crate::utils::{
    dice::Die,
    save::{Save, SaveType},
};

use super::{damage::DamageRoll, save_based::SaveBasedAttack};

#[derive(Clone)]
pub struct Spell {
    save_type: SaveType,
    half_on_success: bool,
    nr_targets: u8,
    damage_dice: Vec<Die>, // todo: damage type, targets, etc.
}

impl Spell {
    pub fn new(
        save_type: SaveType,
        half_on_success: bool,
        nr_targets: u8,
        damage_dice: Vec<Die>,
    ) -> Spell {
        Self {
            save_type,
            half_on_success,
            nr_targets,
            damage_dice,
        }
    }

    pub fn save_type(&self) -> &SaveType {
        &self.save_type
    }

    pub fn nr_targets(&self) -> u8 {
        self.nr_targets
    }

    pub fn half_on_success(&self) -> bool {
        self.half_on_success
    }

    pub fn damage_dice(&self) -> Vec<Die> {
        self.damage_dice.clone()
    }

    pub fn to_spell_based_attack(self, dc: i16) -> SaveBasedAttack {
        SaveBasedAttack::new(
            Save::new(self.save_type, dc),
            self.nr_targets,
            self.half_on_success,
            DamageRoll::new(self.damage_dice, 0),
        )
    }
}
