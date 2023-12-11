use crate::{
    attack::weapon::Weapon,
    character::ability::AbilityModifiers,
    utils::{
        dice::{beats_dc, is_natural_20, Die},
        rollable::Rollable,
    },
};

use super::damage::{Damage, DamageRoll};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum HitResult {
    Miss,
    Hit,
    Critical,
}

#[derive(Clone, Debug)]
pub struct Attack {
    attack_bonus: i16,
    damage: DamageRoll,
}

impl Attack {
    pub fn new(attack_bonus: i16, damage: DamageRoll) -> Self {
        Attack {
            attack_bonus,
            damage: damage,
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

    pub fn calculate_damage(&self, hit_result: HitResult) -> Damage {
        match hit_result {
            HitResult::Miss => Damage::NONE,
            HitResult::Hit => self.damage.calculate_regular(),
            HitResult::Critical => self.damage.calculate_crit(),
        }
    }

    pub(crate) fn roll_attack_with_damage(&self, ac: i16) -> Damage {
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
) -> Attack {
    let attack_bonus = attack_bonus(
        &weapon,
        stats.ability_modifiers(),
        stats.proficiency_bonus(),
    );
    let damage_bonus = weapon_ability_modifier(&weapon, stats.ability_modifiers());
    Attack::new(
        attack_bonus,
        DamageRoll::new(weapon.damage_dice().to_vec(), damage_bonus),
    )
}
