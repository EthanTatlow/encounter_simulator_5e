use crate::weapons::weapons::{WeaponType, WeaponDamage};
use crate::attack::{Attack, HitResult, from_weapon_and_stats};


pub struct StaticPlayerStats {
    strength: u8,
    dexterity: u8,
    proficiency_bonus: u8,
}

impl StaticPlayerStats {
    pub fn strength(&self) -> u8 {
        self.strength
    }

    pub fn dexterity(&self) -> u8 {
        self.dexterity
    }

    pub fn proficiency_bonus(&self) -> u8 {
        self.proficiency_bonus
    }
}

pub struct Player {
    weapon_attack: Attack<WeaponDamage>,
    hit_points: u8,
    stats: StaticPlayerStats
}

impl Player {
    pub fn new(
        weapon_type: WeaponType,
        strength: u8,
        dexterity: u8,
        hit_points: u8,
        proficiency_bonus: u8,
    ) -> Self {
        let stats = StaticPlayerStats{
            strength, dexterity, proficiency_bonus
        };
        let weapon_attack = from_weapon_and_stats(weapon_type.weapon(), &stats);
        Player {
            weapon_attack,
            stats,
            hit_points,
        }
    }

    pub fn roll_attack_with_damage(&self, enemy_armor_class: i16) -> (HitResult, u16) {
        let hit_result = self.weapon_attack.roll_attack(enemy_armor_class);
        let total_damage = self.weapon_attack.calculate_total_damage(hit_result);
        (hit_result, total_damage)
    }

    pub fn mean_damage(&self, enemy_armor_class: i16) -> f32 {
        self.weapon_attack.mean_damage(enemy_armor_class)
    }
}
