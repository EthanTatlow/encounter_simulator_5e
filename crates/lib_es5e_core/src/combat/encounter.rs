use std::{cell::RefCell, rc::Rc};

use crate::stats::Stats;

use super::{
    action::Action,
    combatant::{ActiveCombatant, Combatant, Target},
};

pub struct Encounter {
    players: Vec<Combatant>,
    enemies: Vec<Combatant>,
}

impl Encounter {
    pub fn new(players: Vec<Combatant>, enemies: Vec<Combatant>) -> Encounter {
        Encounter { players, enemies }
    }

    pub fn run<T: Stats>(&self, stats: &mut T) {
        let players: Vec<Rc<RefCell<Combatant>>> = self
            .players
            .to_vec()
            .into_iter()
            .map(|x| Rc::new(RefCell::new(x)))
            .collect();
        let enemies: Vec<Rc<RefCell<Combatant>>> = self
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

fn run_round(players: &[Rc<RefCell<Combatant>>], enemies: &[Rc<RefCell<Combatant>>]) {
    // simplifying assumption: first players, then enemies. no initiativ
    take_actions(players, enemies);
    take_actions(enemies, players);
}

fn take_actions(attackers: &[Rc<RefCell<Combatant>>], targets: &[Rc<RefCell<Combatant>>]) {
    let actions: Vec<Action> = attackers
        .iter()
        .map(|a| a.borrow_mut().take_action())
        .collect();
    actions.iter().for_each(|a| a.execute(attackers, targets));
}

fn all_defeated(combatants: &[Rc<RefCell<Combatant>>]) -> bool {
    combatants.iter().all(|p| !p.borrow().is_conscious())
}

fn count_survivors(combatants: &[Rc<RefCell<Combatant>>]) -> usize {
    combatants
        .iter()
        .filter(|p| p.borrow().is_conscious())
        .count()
}
