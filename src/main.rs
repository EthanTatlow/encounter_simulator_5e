pub mod attack;
pub mod character;
pub mod combat;
pub mod utils;

use attack::{attack::Attack, damage::DamageRoll, spell::Spell, weapon::WeaponType};
use character::character::Character;
use combat::{
    action_selection::{ActionSelection, StatefulAction},
    participant::{CharacterWithActionQueue, Participant},
};
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
    (0..9)
        .into_iter()
        .map(|_| {
            Character::new(
                WeaponType::Longsword,
                AbilityModifiers::new(2, 0, 0, 0, 0, 0),
                14,
                3,
            )
        })
        .map(|c| {
            let attacks = c.get_attacks().clone();
            Participant::Character(
                c,
                ActionSelection::new_default_only(combat::action::Action::MultipleAttacks(attacks)),
            )
        })
        .collect()
}

fn get_casters() -> Vec<Participant> {
    let firebolt = Attack::new(5, DamageRoll::new(vec![Die::D10], 0));
    let spell = Spell::new(SaveType::DEX, true, 3, vec![Die::D6; 3]);
    (0..1)
        .into_iter()
        .map(|_| Character::new_caster(spell.clone(), AbilityModifiers::default(), 16, 70))
        .map(|c| {
            let spell = c.get_spells().first().unwrap().clone();
            Participant::Character(
                c,
                ActionSelection::new(
                    combat::action::Action::MultipleAttacks(vec![
                        firebolt.clone(),
                        firebolt.clone(),
                    ]),
                    vec![StatefulAction::new_with_charges(
                        combat::action::Action::SaveBasedAttack(spell),
                        3,
                    )],
                ),
            )
        })
        .collect()
}
