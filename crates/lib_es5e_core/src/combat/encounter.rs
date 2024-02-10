use std::{cell::RefCell, rc::Rc};

use crate::stats::Stats;

use super::{
    action::Action,
    participant::{ActiveParticipant, Damageable, Participant},
};

pub struct Encounter {
    players: Vec<Participant>,
    enemies: Vec<Participant>,
}

impl Encounter {
    pub fn new(players: Vec<Participant>, enemies: Vec<Participant>) -> Encounter {
        Encounter { players, enemies }
    }

    pub fn run<T: Stats>(&self, stats: &mut T) {
        let players: Vec<Rc<RefCell<Participant>>> = self
            .players
            .to_vec()
            .into_iter()
            .map(|x| Rc::new(RefCell::new(x)))
            .collect();
        let enemies: Vec<Rc<RefCell<Participant>>> = self
            .enemies
            .to_vec()
            .into_iter()
            .map(|x| Rc::new(RefCell::new(x)))
            .collect();

        loop {
            run_round(&players, &enemies);
            stats.record_round();
            if all_defeated(&players) {
                break;
            }
            if all_defeated(&enemies) {
                let nr_survivors = count_survivors(&players);
                stats.record_win(nr_survivors);
                break;
            }
        }
    }
}

fn run_round(players: &[Rc<RefCell<Participant>>], enemies: &[Rc<RefCell<Participant>>]) {
    // simplifying assumption: first players, then enemies. no initiativ
    take_actions(players, enemies);
    take_actions(enemies, players);
}

fn take_actions(attackers: &[Rc<RefCell<Participant>>], targets: &[Rc<RefCell<Participant>>]) {
    let actions: Vec<Action> = attackers
        .iter()
        .map(|a| a.borrow_mut().take_action())
        .collect();
    actions.iter().for_each(|a| a.execute(attackers, targets));
}

fn all_defeated(participants: &[Rc<RefCell<Participant>>]) -> bool {
    participants.iter().all(|p| !p.borrow().is_conscious())
}

fn count_survivors(participants: &[Rc<RefCell<Participant>>]) -> usize {
    participants
        .iter()
        .filter(|p| p.borrow().is_conscious())
        .count()
}
