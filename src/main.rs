pub mod attack;
pub mod character;
pub mod combat;
pub mod utils;

use attack::{spell::Spell, weapon::WeaponType};
use character::character::Character;
use combat::participant::{CharacterWithActionQueue, Participant};
use utils::{dice::Die, save::SaveType};

use crate::{
    character::ability::AbilityModifiers,
    combat::{participant::Damageable, round},
};

fn main() {
    let mut group1_wins = 0;
    let mut nr_rounds_sum = 0;
    let repetitions = 1000;

    for _ in 0..repetitions {
        let mut group1 = get_fighters();
        let mut group2 = get_casters();

        let mut nr_rounds = 0;
        loop {
            nr_rounds += 1;
            round::run_round(&mut group1, &mut group2);
            if group1.iter().all(|c| !c.is_conscious()) {
                break;
            }
            if group2.iter().all(|c| !c.is_conscious()) {
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

fn get_fighters() -> Vec<Participant> {
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
        .map(|c| {
            let attacks = c.get_attacks().clone();
            Participant::Character(CharacterWithActionQueue::new(
                c,
                combat::action::Action::MultipleAttacks(attacks),
            ))
        })
        .collect()
}

fn get_casters() -> Vec<Participant> {
    let spell = Spell::new(SaveType::DEX, true, 3, vec![Die::D6; 3]);
    (0..10)
        .into_iter()
        .map(|_| Character::new_caster(spell.clone(), AbilityModifiers::default(), 15, 9))
        .map(|c| {
            let spell = c.get_spells().first().unwrap().clone();
            Participant::Character(CharacterWithActionQueue::new(
                c,
                combat::action::Action::SaveBasedAttack(spell),
            ))
        })
        .collect()
}
