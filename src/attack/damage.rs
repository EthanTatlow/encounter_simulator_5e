use std::cmp;

use crate::utils::{dice::Die, rollable::roll_sum};

#[derive(Clone)]
pub struct DamageRoll {
    dice: Vec<Die>,
    modifier: i16,
}

pub struct Damage {
    amount: u32,
}

impl Damage {
    pub const NONE: Self = Self { amount: 0 };
    pub fn half(self) -> Self {
        Self {
            amount: self.amount / 2,
            ..self
        }
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }
}

impl DamageRoll {
    pub fn new(dice: Vec<Die>, modifier: i16) -> DamageRoll {
        DamageRoll { dice, modifier }
    }

    pub fn calculate_regular(&self) -> Damage {
        let total_dice_roll = roll_sum(&self.dice);
        Damage {
            amount: cmp::max(0, total_dice_roll as i64 + self.modifier as i64) as u32,
        }
    }

    pub fn calculate_crit(&self) -> Damage {
        let regular_damage = self.calculate_regular();
        Damage {
            amount: regular_damage.amount + roll_sum(&self.dice),
            ..regular_damage
        }
    }
}
