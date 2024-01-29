// This should include spells, spell-like effects, special abilities, etc.

use crate::utils::{
    dice::{beats_dc, Die},
    rollable::Rollable,
    save::Save,
};

use super::{
    damage::{Damage, DamageRoll},
    spell::Spell,
};

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

pub fn from_spell_and_stats(
    spell: Spell,
    stats: &crate::character::character::StaticStats,
) -> SaveBasedAttack {
    // TODO: find spell-casting ability
    let dc = 8 + stats.proficiency_bonus() as i16 + stats.ability_modifiers().int();

    SaveBasedAttack::new(
        Save::new(spell.save_type().clone(), dc),
        spell.nr_targets(),
        spell.half_on_success(),
        DamageRoll::new(spell.damage_dice(), 0),
    )
}
