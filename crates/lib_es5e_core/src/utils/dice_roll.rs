use std::{cmp, str::FromStr};

use super::{dice::Die, rollable::roll_sum};

#[derive(Clone, Debug, Default)]
pub struct DiceRoll {
    dice: Vec<Die>,
    modifier: i16,
}

impl DiceRoll {
    pub fn new(dice: Vec<Die>, modifier: i16) -> Self {
        Self { dice, modifier }
    }

    pub fn with_mod(&self) -> u32 {
        let total_dice_roll = roll_sum(&self.dice);
        cmp::max(0, total_dice_roll as i32 + self.modifier as i32) as u32
    }

    pub fn without_mod(&self) -> u32 {
        roll_sum(&self.dice)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDiceRoll;

impl FromStr for DiceRoll {
    type Err = ParseDiceRoll;
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
        return Ok(Self {
            dice: vec![die_type.unwrap(); nr_dice],
            modifier: signed_modifier,
        });
    }
}
