use std::cmp::min;

use crate::attack::{
    attack::{from_weapon_and_stats, Attack},
    weapon::{WeaponDamage, WeaponType},
};

use super::effect::NegativeEffect;
use super::save::SaveModifiers;

#[derive(Clone)]
pub struct StaticStats {
    strength: u8,
    dexterity: u8,
    ac: i16,
    proficiency_bonus: u8,
    saves: SaveModifiers,
}

impl StaticStats {
    pub fn strength(&self) -> u8 {
        self.strength
    }

    pub fn dexterity(&self) -> u8 {
        self.dexterity
    }

    pub fn proficiency_bonus(&self) -> u8 {
        self.proficiency_bonus
    }

    pub fn saves(&self) -> &SaveModifiers {
        &self.saves
    }
}

#[derive(Clone)]
pub struct Character {
    weapon_attack: Attack<WeaponDamage>,
    hit_points: u16,
    stats: StaticStats,
}

impl Character {
    pub fn new(
        weapon_type: WeaponType,
        strength: u8,
        dexterity: u8,
        ac: i16,
        hit_points: u16,
        proficiency_bonus: u8,
    ) -> Self {
        let stats = StaticStats {
            ac,
            strength,
            dexterity,
            proficiency_bonus,
            saves: SaveModifiers::default(),
        };
        let weapon_attack = from_weapon_and_stats(weapon_type.weapon(), &stats);
        Character {
            weapon_attack,
            stats,
            hit_points,
        }
    }

    pub fn get_effects_on_enemies(&self) -> Vec<impl NegativeEffect> {
        return vec![self.weapon_attack.clone()];
    }

    pub fn ac(&self) -> i16 {
        return self.stats.ac;
    }

    pub fn roll_attack_with_damage(&self, enemy_armor_class: i16) -> u16 {
        self.weapon_attack
            .roll_attack_with_damage(enemy_armor_class)
    }

    pub fn mean_damage(&self, enemy_armor_class: i16) -> f32 {
        self.weapon_attack.mean_damage_against_ac(enemy_armor_class)
    }

    pub(crate) fn take_damage(&mut self, total_damage: u16) {
        self.hit_points -= min(total_damage, self.hit_points)
    }

    pub(crate) fn is_dead(&self) -> bool {
        return self.hit_points == 0;
    }

    pub fn saves(&self) -> &SaveModifiers {
        &self.stats.saves
    }
}
