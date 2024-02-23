use std::str::FromStr;

use crate::utils::{
    dice::Die,
    dice_roll::{DiceRoll, ParseDiceRoll},
};

#[derive(Clone, Debug, Default)]
pub struct DamageRoll(DiceRoll);

impl DamageRoll {
    pub fn new(dice: Vec<Die>, modifier: i16) -> Self {
        Self(DiceRoll::new(dice, modifier))
    }

    pub fn calculate_regular(&self) -> Damage {
        Damage {
            amount: self.0.with_mod(),
        }
    }

    pub fn calculate_crit(&self) -> Damage {
        let regular_damage = self.calculate_regular();
        Damage {
            amount: regular_damage.amount + self.0.without_mod(),
            ..regular_damage
        }
    }
}

#[derive(Clone, Debug)]
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

impl FromStr for DamageRoll {
    type Err = ParseDiceRoll;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(DiceRoll::from_str(s)?))
    }
}
