use crate::attack::{attack::Attack, save_based::SaveBasedAttack};

use crate::character::effect::NegativeEffect;

use super::participant::Participant;
use super::targeting::{select_random_target, select_random_targets};

#[derive(Clone)]
pub enum Action {
    MultipleAttacks(Vec<Attack>),
    SaveBasedAttack(SaveBasedAttack),
}

impl Action {
    pub fn execute(&self, _allies: &mut [Participant], enemies: &mut [Participant]) {
        match self {
            Action::MultipleAttacks(atks) => execute_attacks(atks, enemies),
            Action::SaveBasedAttack(atk) => execute_save_based_attack(atk, enemies),
        }
    }
}

fn execute_attacks(atks: &Vec<Attack>, enemies: &mut [Participant]) {
    atks.iter().for_each(|atk| {
        if let Some(target) = select_random_target(enemies) {
            atk.apply(target);
        }
    })
}

fn execute_save_based_attack(atk: &SaveBasedAttack, enemies: &mut [Participant]) {
    let targets_iter = select_random_targets(atk.nr_targets() as usize, enemies);
    targets_iter.for_each(|enemy| atk.apply(enemy));
}
