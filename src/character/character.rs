use std::cmp::min;

use crate::attack::{
    attack::{from_weapon_and_stats, Attack},
    damage::Damage,
    save_based::{from_spell_and_stats, SaveBasedAttack},
    spell::Spell,
    weapon::WeaponType,
};

use super::ability::AbilityModifiers;
use super::save::SaveModifiers;

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
    weapon_attacks: Vec<Attack>,
    spell_attacks: Vec<SaveBasedAttack>,
    hit_points: u32,
    stats: StaticStats,
}

impl Character {
    pub fn new(
        weapon_type: WeaponType,
        ability_modifiers: AbilityModifiers,
        ac: i16,
        hit_points: u32,
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

    pub fn new_caster(
        spell: Spell,
        ability_modifiers: AbilityModifiers,
        ac: i16,
        hit_points: u32,
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

    pub fn get_attacks(&self) -> &Vec<Attack> {
        &self.weapon_attacks
    }

    pub fn get_spells(&self) -> &Vec<SaveBasedAttack> {
        &self.spell_attacks
    }

    pub fn ac(&self) -> i16 {
        return self.stats.ac;
    }

    pub(crate) fn take_damage(&mut self, damage: Damage) {
        self.hit_points -= min(damage.amount(), self.hit_points)
    }

    pub(crate) fn is_dead(&self) -> bool {
        return self.hit_points == 0;
    }

    pub fn saves(&self) -> &SaveModifiers {
        &self.stats.save_modifiers
    }
}
