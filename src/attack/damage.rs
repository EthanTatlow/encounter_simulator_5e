use std::{cmp, str::FromStr};

use crate::utils::{dice::Die, rollable::roll_sum};

#[derive(Clone, Debug)]
pub struct DamageRoll {
    dice: Vec<Die>,
    modifier: i16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDamageRoll;

impl FromStr for DamageRoll {
    type Err = ParseDamageRoll;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalised = s.replace(" ", "").replace("d", "D");
        let mut components = normalised.split(&['+', '-']); // TODO: handle minus
        let dice = components.next();

        let mut dice_split_iterator = dice.unwrap().split("D");
        let nr_dice = dice_split_iterator
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let die_type = match dice_split_iterator.next().unwrap() {
            "2" => Some(Die::D2),
            "4" => Some(Die::D4),
            "6" => Some(Die::D6),
            "8" => Some(Die::D8),
            "10" => Some(Die::D10),
            "12" => Some(Die::D12),
            "20" => Some(Die::D20),
            _ => None,
        };
        let modifier: i16 = components.next().unwrap_or("0").parse().unwrap();
        let signed_modifier = if normalised.contains('-') {
            -modifier
        } else {
            modifier
        };
        return Ok(DamageRoll::new(
            vec![die_type.unwrap(); nr_dice],
            signed_modifier,
        ));
    }
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
