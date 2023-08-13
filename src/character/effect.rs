use crate::{
    attack::{attack::Attack, save_based::SaveBasedAttack},
    combat::participant::Damageable,
};

pub trait NegativeEffect {
    fn number_of_targets(&self) -> u8;
    fn apply<T: Damageable>(&self, target: &mut T);
}

impl NegativeEffect for Attack {
    fn number_of_targets(&self) -> u8 {
        1
    }

    fn apply<T: Damageable>(&self, target: &mut T) {
        let damage: crate::attack::damage::Damage = self.roll_attack_with_damage(target.ac());
        if damage.amount() > 0 {
            target.take_damage(damage)
        }
    }
}

impl NegativeEffect for SaveBasedAttack {
    fn number_of_targets(&self) -> u8 {
        self.nr_targets()
    }

    fn apply<T: Damageable>(&self, target: &mut T) {
        let save_modifier: i16 = target.saves().modifier(self.save().save_type());
        let damage = self.roll_save(save_modifier);
        if damage.amount() > 0 {
            target.take_damage(damage)
        }
    }
}
