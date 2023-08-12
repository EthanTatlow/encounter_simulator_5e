// This should include spells, spell-like effects, special abilities, etc.

use crate::utils::{
    dice::{beats_dc, Die},
    probability::Meanable,
    rollable::Rollable,
    save::Save,
};

use super::damage::Damage;

pub struct SaveBasedAttack<T: Rollable<u32> + Meanable> {
    save: Save,
    nr_targets: u8,
    half_on_success: bool,
    damage: Damage<T>,
}

impl<T: Rollable<u32> + Meanable> SaveBasedAttack<T> {
    pub fn roll_save(&self, save_bonus: i16) -> u16 {
        let roll = Die::D20.roll();
        let take_full_damage = !beats_dc(roll as i16, self.save.dc() as i16 - save_bonus);

        if take_full_damage {
            self.damage.calculate_regular()
        } else if self.half_on_success {
            self.damage.calculate_regular() / 2
        } else {
            0
        }
    }

    pub fn save(&self) -> &Save {
        &self.save
    }

    pub fn nr_targets(&self) -> u8 {
        self.nr_targets
    }
}
