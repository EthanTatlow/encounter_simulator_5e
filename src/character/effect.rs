use crate::{
    attack::{attack::Attack, save_based::SaveBasedAttack, weapon::Weapon},
    utils::{probability::Meanable, rollable::Rollable},
};

use super::character::Character;

pub trait NegativeEffect {
    fn number_of_targets(&self) -> u8;
    fn apply(&self, character: &mut Character);
}

impl<T: Rollable<u32> + Meanable> NegativeEffect for Attack<T> {
    fn number_of_targets(&self) -> u8 {
        1
    }

    fn apply(&self, character: &mut Character) {
        let damage_dealt = self.roll_attack_with_damage(character.ac());
        if damage_dealt > 0 {
            character.take_damage(damage_dealt)
        }
    }
}

impl<T: Rollable<u32> + Meanable> NegativeEffect for SaveBasedAttack<T> {
    fn number_of_targets(&self) -> u8 {
        self.nr_targets()
    }

    fn apply(&self, character: &mut Character) {
        let save_bonus = character.saves().modifier(self.save().save_type());
        let damage_dealt = self.roll_save(save_bonus);
        if damage_dealt > 0 {
            character.take_damage(damage_dealt)
        }
    }
}
