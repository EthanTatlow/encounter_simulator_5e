use std::cmp::min;

use crate::attack::{
    attack::{from_weapon_and_stats, Attack},
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
    weapon_attack: Attack<WeaponDamage>,
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
            ability_modifiers,
            save_modifiers: SaveModifiers::default(),
            proficiency_bonus: 2,
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
