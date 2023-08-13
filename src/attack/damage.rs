use std::cmp;

use crate::utils::{dice::Die, probability::mean_sum, rollable::roll_sum};

#[derive(Clone)]
pub struct Damage {
    dice: Vec<Die>,
    modifier: i16,
}

impl Damage {
    pub(crate) fn new(dice: Vec<Die>, modifier: i16) -> Damage {
        Damage { dice, modifier }
    }

    pub fn calculate_regular(&self) -> u16 {
        let total_dice_roll = roll_sum(&self.dice);
        cmp::max(0, total_dice_roll as i32 + self.modifier as i32) as u16
    }

    pub fn calculate_crit(&self) -> u16 {
        self.calculate_regular() + roll_sum(&self.dice) as u16
    }

    pub fn mean_on_hit(&self) -> f32 {
        mean_sum(&self.dice) + self.modifier as f32
    }

    pub fn mean_on_crit(&self) -> f32 {
        self.mean_on_hit() + mean_sum(&self.dice)
    }
}
