pub mod character;
pub mod utils;
pub mod attack;

use character::character::Character;
use attack::weapon::WeaponType;

fn main() {
    let weapon_type = WeaponType::BattleAxe;
    println!("Weapon: {:?}", weapon_type);

    let player = Character::new(weapon_type, 16, 14, 30, 2);

    let (hit_result, total_damage) = player.roll_attack_with_damage(15);

    println!("Mean Damage: {}", player.mean_damage(15));
    println!("Hit Result: {:?}", hit_result);
    println!("Total Damage: {}", total_damage);
}
