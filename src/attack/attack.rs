use std::cmp;

use crate::{
    attack::weapon::{Weapon, WeaponDamage},
    utils::{
        dice::{beats_dc, is_natural_20, Die},
        probability::Meanable,
        rollable::Rollable,
    },
};

use super::damage::Damage;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum HitResult {
    Miss,
    Hit,
    Critical,
}

#[derive(Clone)]
pub struct Attack<T: Rollable<u32> + Meanable> {
    attack_bonus: i8,
    damage: Damage<T>,
}

impl<T: Rollable<u32> + Meanable> Attack<T> {
    pub fn new(attack_bonus: i8, damage: T, bonus: i8) -> Self {
        Attack {
            attack_bonus,
            damage: Damage::new(damage, bonus),
        }
    }

    pub fn roll_attack(&self, ac: i16) -> HitResult {
        let hit_roll = Die::D20.roll();
        let is_crit = is_natural_20(hit_roll);
        let is_hit = beats_dc(hit_roll as i32, ac as i32 - (self.attack_bonus as i32));

        match (is_hit, is_crit) {
            (_, true) => HitResult::Critical,
            (true, _) => HitResult::Hit,
            (false, _) => HitResult::Miss,
        }
    }

    pub fn calculate_damage(&self, hit_result: HitResult) -> u16 {
        match hit_result {
            HitResult::Miss => 0,
            HitResult::Hit => self.damage.calculate_regular(),
            HitResult::Critical => self.damage.calculate_crit(),
        }
    }

    pub fn mean_damage_against_ac(&self, enemy_armor_class: i16) -> f32 {
        let effective_ac = enemy_armor_class - self.attack_bonus as i16;
        let reg_hit_prob = if effective_ac >= 20 {
            0.0
        } else {
            cmp::min(20 - effective_ac, 19) as f32 * 0.05
        };

        reg_hit_prob * self.mean_damage_on_hit() + 0.05 * self.mean_damage_on_crit()
    }

    fn mean_damage_on_hit(&self) -> f32 {
        self.damage.mean_on_hit()
    }

    fn mean_damage_on_crit(&self) -> f32 {
        self.damage.mean_on_hit()
    }
}

fn weapon_ability_modifier(weapon: &Weapon, strength: u8, dex: u8) -> i8 {
    if weapon.is_finesse() {
        (dex / 2) as i8 - 5
    } else {
        (strength / 2) as i8 - 5
    }
}

fn attack_bonus(weapon: &Weapon, strength: u8, dex: u8, prof: u8) -> i8 {
    let ability_modifier = weapon_ability_modifier(weapon, strength, dex);
    prof as i8 + ability_modifier
}

pub(crate) fn from_weapon_and_stats(
    weapon: Weapon,
    stats: &crate::character::character::StaticStats,
) -> Attack<WeaponDamage> {
    let attack_bonus = attack_bonus(
        &weapon,
        stats.strength(),
        stats.dexterity(),
        stats.proficiency_bonus(),
    );
    let damage_bonus = weapon_ability_modifier(&weapon, stats.strength(), stats.dexterity());
    Attack::new(attack_bonus, WeaponDamage::new(weapon), damage_bonus)
}
