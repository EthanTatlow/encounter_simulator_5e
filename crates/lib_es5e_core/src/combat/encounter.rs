use std::{cell::RefCell, rc::Rc};

use rand::{seq::SliceRandom, thread_rng};

use crate::{combatant::combatant::Combatant, statistics::Statistics};

#[derive(Debug, Clone)]
pub struct Encounter {
    players: Vec<Combatant>,
    enemies: Vec<Combatant>,
}

#[derive(Debug, Clone)]
pub struct CombatantWithRelations {
    combatant: Rc<RefCell<Combatant>>,
    allies: Vec<Rc<RefCell<Combatant>>>,
    enemies: Vec<Rc<RefCell<Combatant>>>,
}

impl Encounter {
    pub fn new(players: Vec<Combatant>, enemies: Vec<Combatant>) -> Encounter {
        Encounter { players, enemies }
    }

    pub fn run<T: Statistics>(&self, stats: &mut T) {
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

        let players_with_relations: Vec<_> = players
            .to_vec()
            .into_iter()
            .map(|combatant| CombatantWithRelations {
                combatant,
                allies: players.to_vec(),
                enemies: enemies.to_vec(),
            })
            .collect();
        let enemies_with_relations: Vec<_> = enemies
            .to_vec()
            .into_iter()
            .map(|combatant| CombatantWithRelations {
                combatant,
                allies: enemies.to_vec(),
                enemies: players.to_vec(),
            })
            .collect();

        let all_combatants = {
            let mut all = [players_with_relations, enemies_with_relations].concat();
            all.shuffle(&mut thread_rng()); // note: not entirely random -> TODO: implement initiative
            all
        };

        loop {
            run_round(&all_combatants);
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

fn run_round(combatants: &[CombatantWithRelations]) {
    for combatant in combatants {
        if combatant.combatant.borrow().is_conscious() {
            let action = combatant.combatant.borrow().first_available_action();
            action.execute(&combatant.allies, &combatant.enemies)
        }
    }
}

//fn take_actions(attackers: &[Rc<RefCell<Combatant>>], targets: &[Rc<RefCell<Combatant>>]) {
//    let actions: Vec<Box<dyn Action>> = attackers
//        .iter()
//        .map(|a| a.borrow_mut().take_action())
//    actions.iter().for_each(|a| a.execute(attackers, targets));
//}

fn all_defeated(combatants: &[Rc<RefCell<Combatant>>]) -> bool {
    combatants.iter().all(|p| !p.borrow().is_conscious())
}

fn count_survivors(combatants: &[Rc<RefCell<Combatant>>]) -> usize {
    combatants
        .iter()
        .filter(|p| p.borrow().is_conscious())
        .count()
}
