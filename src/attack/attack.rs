use std::cmp;

use crate::{attack::weapon::{Weapon, WeaponDamage}, utils::{dice::{Die, is_natural_20, beats_dc}, probability::Meanable, rollable::Rollable}};


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum HitResult {
    Miss,
    Hit,
    Critical,
}

pub struct Attack<T: Rollable<u32> + Meanable> {
    attack_bonus: i8,
    damage_dice: T,
    damage_bonus: i8,
}

impl <T: Rollable<u32> + Meanable> Attack<T> {
    pub fn new(attack_bonus: i8, damage: T, bonus: i8) -> Self {
        Attack {
            attack_bonus,
            damage_dice: damage,
            damage_bonus: bonus,
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

    pub fn calculate_total_damage(&self, hit_result: HitResult) -> u16 {
        match hit_result {
            HitResult::Miss => 0,
            HitResult::Hit => {
                let total_dice_roll = self.damage_dice.roll();
                cmp::max(0, total_dice_roll as i32 + self.attack_bonus as i32) as u16
            }
            HitResult::Critical => {
                let total_dice_roll = self.damage_dice.roll();
                Attack::calculate_total_damage(self, HitResult::Hit) + total_dice_roll as u16
            }
        }
    }

    pub fn mean_damage(&self, enemy_armor_class: i16) -> f32 {
        let effective_ac = enemy_armor_class - self.attack_bonus as i16;
        let reg_hit_prob = if effective_ac >= 20 {
            0.0
        } else {
            cmp::min( 20 - effective_ac, 19) as f32 * 0.05
        };

        reg_hit_prob * self.mean_damage_on_hit() + 0.05 * self.mean_damage_on_crit()
    }

    fn mean_damage_on_hit(&self) -> f32 {
        self.damage_dice.mean() + self.damage_bonus as f32
    } 
    
    fn mean_damage_on_crit(&self) -> f32 {
        self.mean_damage_on_hit() + self.damage_dice.mean()
    }

}


fn weapon_ability_modifier(weapon: &Weapon, strength: u8, dex: u8) -> i8 {
    if weapon.is_finesse() {
        (dex / 2) as i8 - 5
    } else {
        (strength / 2) as i8  - 5
    }
}

fn attack_bonus(weapon: &Weapon, strength: u8, dex: u8, prof: u8) -> i8 {
    let ability_modifier = weapon_ability_modifier(weapon, strength, dex);
    prof as i8 + ability_modifier
}

pub(crate) fn from_weapon_and_stats(weapon: Weapon, stats: &crate::character::character::StaticStats) -> Attack<WeaponDamage> {
    let attack_bonus = attack_bonus(&weapon, stats.strength(), stats.dexterity(), stats.proficiency_bonus());
    let damage_bonus = weapon_ability_modifier(&weapon, stats.strength(), stats.dexterity());
    Attack::new(attack_bonus, WeaponDamage::new(weapon), damage_bonus)
}