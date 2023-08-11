use std::cmp;

use crate::utils::{probability::Meanable, rollable::Rollable};

#[derive(Clone)]
pub struct Damage<T: Rollable<u32> + Meanable> {
    dice: T,
    bonus: i8,
}

impl<T: Rollable<u32> + Meanable> Damage<T> {
    pub(crate) fn new(dice: T, bonus: i8) -> Damage<T> {
        Damage { dice, bonus }
    }

    pub fn calculate_regular(&self) -> u16 {
        let total_dice_roll = self.dice.roll();
        cmp::max(0, total_dice_roll as i32 + self.bonus as i32) as u16
    }

    pub fn calculate_crit(&self) -> u16 {
        self.calculate_regular() + self.dice.roll() as u16
    }

    pub fn mean_on_hit(&self) -> f32 {
        self.dice.mean() + self.bonus as f32
    }

    pub fn mean_on_crit(&self) -> f32 {
        self.mean_on_hit() + self.dice.mean()
    }
}
