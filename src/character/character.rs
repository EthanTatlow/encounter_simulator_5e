use std::cmp::min;

use rand::{thread_rng, Rng};

use crate::attack::{
    attack::{from_weapon_and_stats, Attack},
    save_based::{from_spell_and_stats, SaveBasedAttack},
    spell::{Spell, SpellDamage},
    weapon::{WeaponDamage, WeaponType},
};

use super::save::SaveModifiers;
use super::{ability::AbilityModifiers, effect::NegativeEffect};

#[derive(Clone)]
pub struct StaticStats {
    ac: i16,
    ability_modifiers: AbilityModifiers,
    save_modifiers: SaveModifiers,
    proficiency_bonus: u8,
}

impl StaticStats {
    pub fn ability_modifiers(&self) -> &AbilityModifiers {
        &self.ability_modifiers
    }

    pub fn saves(&self) -> &SaveModifiers {
        &self.save_modifiers
    }

    pub fn proficiency_bonus(&self) -> u8 {
        self.proficiency_bonus
    }
}

#[derive(Clone)]
pub struct Character {
    weapon_attacks: Vec<Attack<WeaponDamage>>,
    spell_attacks: Vec<SaveBasedAttack<SpellDamage>>,
    hit_points: u16,
    stats: StaticStats,
}

impl Character {
    pub fn new(
        weapon_type: WeaponType,
        ability_modifiers: AbilityModifiers,
        ac: i16,
        hit_points: u16,
    ) -> Self {
        let stats = StaticStats {
            ac,
            ability_modifiers: ability_modifiers.clone(),
            save_modifiers: SaveModifiers::from_modifiers(ability_modifiers),
            proficiency_bonus: 2,
        };
        let weapon_attack = from_weapon_and_stats(weapon_type.weapon(), &stats);
        Character {
            weapon_attacks: vec![weapon_attack],
            spell_attacks: vec![],
            stats,
            hit_points,
        }
    }

    pub fn new_wiz(
        spell: Spell,
        ability_modifiers: AbilityModifiers,
        ac: i16,
        hit_points: u16,
    ) -> Self {
        let stats = StaticStats {
            ac,
            ability_modifiers,
            save_modifiers: SaveModifiers::default(),
            proficiency_bonus: 2,
        };

        Character {
            weapon_attacks: vec![],
            spell_attacks: vec![from_spell_and_stats(spell, &stats)],
            stats,
            hit_points,
        }
    }

    pub fn get_effects_on_enemies(&self) -> Vec<Box<dyn NegativeEffect>> {
        if !self.spell_attacks.is_empty() {
            let spell_attack =
                self.spell_attacks[thread_rng().gen_range(0..self.spell_attacks.len())].clone();
            return vec![Box::new(spell_attack)];
        }
        self.weapon_attacks
            .iter()
            .map(|atk| Box::new(atk.clone()) as Box<dyn NegativeEffect>)
            .collect()
    }

    pub fn ac(&self) -> i16 {
        return self.stats.ac;
    }

    pub(crate) fn take_damage(&mut self, total_damage: u16) {
        self.hit_points -= min(total_damage, self.hit_points)
    }

    pub(crate) fn is_dead(&self) -> bool {
        return self.hit_points == 0;
    }

    pub fn saves(&self) -> &SaveModifiers {
        &self.stats.save_modifiers
    }
}
