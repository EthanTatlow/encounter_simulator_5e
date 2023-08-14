pub mod attack;
pub mod character;
pub mod combat;
pub mod utils;

use attack::{attack::Attack, damage::DamageRoll, spell::Spell, weapon::WeaponType};
use character::{character::Character, save::SaveModifiers};
use combat::{
    action_selection::{ActionSelection, StatefulAction},
    participant::Participant,
};
use utils::{dice::Die, save::SaveType};

use crate::{
    character::ability::AbilityModifiers,
    combat::{action::Action, participant::Damageable, round},
};

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let mut group1_wins = 0;
    let mut nr_rounds_sum = 0;
    let repetitions = 1000;

    for _ in 0..repetitions {
        let mut group1 = get_fighters();
        let mut group2 = get_dragon();

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

    let elapsed = now.elapsed();
    println!("Program duration: {:.2?}", elapsed);
}

fn get_fighters() -> Vec<Participant> {
    (0..5)
        .into_iter()
        .map(|_| {
            Character::new(
                WeaponType::Longsword,
                AbilityModifiers::new(10, 10, 0, 0, 0, 0),
                18,
                120,
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

fn get_dragon() -> Vec<Participant> {
    let bite = Attack::new(15, DamageRoll::new(vec![Die::D10; 2], 8));
    let claws = Attack::new(15, DamageRoll::new(vec![Die::D6; 2], 8));
    let breath_weapon = Spell::new(SaveType::DEX, true, 3, vec![Die::D8; 15]);
    let action_selection = ActionSelection::new(
        Action::MultipleAttacks(vec![bite, claws.clone(), claws]),
        vec![StatefulAction::new_recharge(
            Action::SaveBasedAttack(breath_weapon.to_spell_based_attack(22)),
            5,
        )],
    );

    let saves = SaveModifiers::new(8, 9, 14, 3, 9, 11);
    let dragon = Participant::new_simple(367, 22, saves, action_selection);

    vec![dragon]
}
