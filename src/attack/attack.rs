use std::cmp;

use crate::{
    attack::weapon::{Weapon, WeaponDamage},
    character::ability::AbilityModifiers,
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
    attack_bonus: i16,
    damage: Damage<T>,
}

impl<T: Rollable<u32> + Meanable> Attack<T> {
    pub fn new(attack_bonus: i16, damage: T, bonus: i16) -> Self {
        Attack {
            attack_bonus,
            damage: Damage::new(damage, bonus),
        }
    }

    pub fn roll_attack(&self, ac: i16) -> HitResult {
        let hit_roll = Die::D20.roll();
        let is_crit = is_natural_20(hit_roll);
        let is_critical_miss = hit_roll == 1;
        let effective_ac = ac as i32 - self.attack_bonus as i32;
        let is_hit = !is_critical_miss && beats_dc(hit_roll as i32, effective_ac);

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
            cmp::min(20 - effective_ac - 1, 18) as f32 * 0.05 // note: subtract 1 to account for fumble
        };

        reg_hit_prob * self.mean_damage_on_hit() + 0.05 * self.mean_damage_on_crit()
    }

    fn mean_damage_on_hit(&self) -> f32 {
        self.damage.mean_on_hit()
    }

    fn mean_damage_on_crit(&self) -> f32 {
        self.damage.mean_on_hit()
    }

    pub(crate) fn roll_attack_with_damage(&self, ac: i16) -> u16 {
        let hit_result = self.roll_attack(ac);
        self.calculate_damage(hit_result)
    }
}

fn weapon_ability_modifier(weapon: &Weapon, ability_modifiers: &AbilityModifiers) -> i16 {
    if weapon.is_finesse() {
        ability_modifiers.dex()
    } else {
        ability_modifiers.str()
    }
}

fn attack_bonus(weapon: &Weapon, ability_modifiers: &AbilityModifiers, prof: u8) -> i16 {
    let ability_modifier = weapon_ability_modifier(weapon, ability_modifiers);
    prof as i16 + ability_modifier
}

pub(crate) fn from_weapon_and_stats(
    weapon: Weapon,
    stats: &crate::character::character::StaticStats,
) -> Attack<WeaponDamage> {
    let attack_bonus = attack_bonus(
        &weapon,
        stats.ability_modifiers(),
        stats.proficiency_bonus(),
    );
    let damage_bonus = weapon_ability_modifier(&weapon, stats.ability_modifiers());
    Attack::new(attack_bonus, WeaponDamage::new(weapon), damage_bonus)
}
