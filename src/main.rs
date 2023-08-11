mod player;
mod utils;
mod weapons;
pub mod attack;

use player::Player;
use weapons::weapons::WeaponType;

fn main() {
    let weapon_type = WeaponType::BattleAxe;
    println!("Weapon: {:?}", weapon_type);

    let player = Player::new(weapon_type, 16, 14, 30, 2);

    let (hit_result, total_damage) = player.roll_attack_with_damage(15);

    println!("Mean Damage: {}", player.mean_damage(15));
    println!("Hit Result: {:?}", hit_result);
    println!("Total Damage: {}", total_damage);
}
