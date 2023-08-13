use crate::utils::{dice::Die, save::SaveType};

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
}
