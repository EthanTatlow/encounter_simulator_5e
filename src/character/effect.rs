use crate::attack::{attack::Attack, save_based::SaveBasedAttack};

use super::character::Character;

pub trait NegativeEffect {
    fn number_of_targets(&self) -> u8;
    fn apply(&self, character: &mut Character);
}

impl NegativeEffect for Attack {
    fn number_of_targets(&self) -> u8 {
        1
    }

    fn apply(&self, character: &mut Character) {
        let damage = self.roll_attack_with_damage(character.ac());
        if damage.amount() > 0 {
            character.take_damage(damage)
        }
    }
}

impl NegativeEffect for SaveBasedAttack {
    fn number_of_targets(&self) -> u8 {
        self.nr_targets()
    }

    fn apply(&self, character: &mut Character) {
        let save_modifier: i16 = character.saves().modifier(self.save().save_type());
        let damage = self.roll_save(save_modifier);
        if damage.amount() > 0 {
            character.take_damage(damage)
        }
    }
}
