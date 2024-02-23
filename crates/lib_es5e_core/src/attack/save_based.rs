// This should include spells, spell-like effects, special abilities, etc.

use crate::{
    action::effect::Effect,
    combatant::combatant::Combatant,
    utils::{
        dice::{beats_dc, Die},
        rollable::Rollable,
        save::Save,
    },
};

use super::damage::{Damage, DamageRoll};

#[derive(Clone, Debug)]
pub struct SaveBasedAttack {
    save: Save,
    nr_targets: usize,
    half_on_success: bool,
    damage: DamageRoll,
}

impl SaveBasedAttack {
    pub fn new(
        save: Save,
        nr_targets: usize,
        half_on_success: bool,
        damage: DamageRoll,
    ) -> SaveBasedAttack {
        Self {
            save,
            nr_targets,
            half_on_success,
            damage,
        }
    }

    pub fn roll_save(&self, save_bonus: i16) -> Damage {
        let roll = Die::D20.roll();
        let take_full_damage = !beats_dc(roll as i16, self.save.dc() as i16 - save_bonus);

        if take_full_damage {
            self.damage.calculate_regular()
        } else if self.half_on_success {
            self.damage.calculate_regular().half()
        } else {
            Damage::NONE
        }
    }

    pub fn save(&self) -> &Save {
        &self.save
    }

    pub fn nr_targets(&self) -> usize {
        self.nr_targets
    }
}

impl Effect for SaveBasedAttack {
    fn number_of_targets(&self) -> usize {
        self.nr_targets()
    }

    fn apply(&self, target: &mut Combatant) {
        let save_modifier: i16 = target.saves().modifier(self.save().save_type());
        let damage = self.roll_save(save_modifier);
        if damage.amount() > 0 {
            target.take_damage(damage)
        }
    }
}
