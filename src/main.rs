pub mod attack;
pub mod character;
pub mod combat;
pub mod utils;

use attack::{spell::Spell, weapon::WeaponType};
use character::character::Character;
use utils::{dice::Die, save::SaveType};

use crate::{character::ability::AbilityModifiers, combat::round};

fn main() {
    let mut group1_wins = 0;
    let mut nr_rounds_sum = 0;
    let repetitions = 1000;

    for _ in 0..repetitions {
        let mut group1 = get_fighters();
        let mut group2 = get_group_2();

        let mut nr_rounds = 0;
        loop {
            nr_rounds += 1;
            round::run_round(&mut group1, &mut group2);
            if group1.iter().all(|c| c.is_dead()) {
                break;
            }
            if group2.iter().all(|c| c.is_dead()) {
                group1_wins += 1;
                break;
            }
        }
        nr_rounds_sum += nr_rounds;
    }

    println!(
        "Group 1 won {} % of the time",
        group1_wins as f32 / repetitions as f32 * 100.0
    );
    println!(
        "Average number of rounds: {}",
        nr_rounds_sum as f32 / repetitions as f32
    );
}

fn get_fighters() -> Vec<Character> {
    (0..20)
        .into_iter()
        .map(|_| {
            Character::new(
                WeaponType::Shortsword,
                AbilityModifiers::new(4, 4, 0, 0, 0, 0),
                14,
                9,
            )
        })
        .collect()
}

fn get_group_2() -> Vec<Character> {
    let spell = Spell::new(SaveType::DEX, true, 3, vec![Die::D6; 3]);
    (0..15)
        .into_iter()
        .map(|_| Character::new_caster(spell.clone(), AbilityModifiers::default(), 15, 9))
        .collect()
}
