pub mod attack;
pub mod character;
pub mod combat;
pub mod utils;

use attack::weapon::WeaponType;
use character::character::Character;

use crate::combat::round;

fn main() {
    let weapon_type = WeaponType::BattleAxe;
    println!("Weapon: {:?}", weapon_type);

    let mut group1wins = 0;
    let repetitions = 10000;
    for _ in 0..repetitions {
        let mut group1 = [
            Character::new(WeaponType::Shortsword, 16, 14, 14, 30, 2),
            Character::new(WeaponType::Scimitar, 16, 14, 14, 30, 2),
        ];

        let mut group2 = [
            Character::new(WeaponType::Longsword, 16, 14, 14, 30, 2),
            Character::new(WeaponType::Greatsword, 16, 14, 14, 30, 2),
        ];

        loop {
            round::run_round(&mut group1, &mut group2);
            if group1.iter().all(|c| c.is_dead()) {
                break;
            }
            if group2.iter().all(|c| c.is_dead()) {
                group1wins += 1;
                break;
            }
        }
    }
    println!(
        "Group 1 won {} % of the time",
        group1wins as f32 / repetitions as f32 * 100.0
    );
}
