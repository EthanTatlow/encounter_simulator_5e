use std::{cell::RefCell, rc::Rc};

use crate::{
    combatant::{combatant::Combatant, config::CombatantConfig},
    statistics::Statistics,
};
use crate::utils::dice::Die::D20;
use crate::utils::rollable::Rollable;

pub struct Encounter {
    players: Vec<CombatantConfig>,
    enemies: Vec<CombatantConfig>,
}

pub type IntMutCombatant = Rc<RefCell<Combatant>>;

pub struct CombatantWithRelations {
    combatant: IntMutCombatant,
    allies: Vec<IntMutCombatant>,
    enemies: Vec<IntMutCombatant>,
}

impl Encounter {
    pub fn new(players: Vec<CombatantConfig>, enemies: Vec<CombatantConfig>) -> Encounter {
        Encounter { players, enemies }
    }

    pub fn run<T: Statistics>(&self, stats: &mut T) {
        let players = self.instantiate_for_run(&self.players);
        let enemies = self.instantiate_for_run(&self.enemies);
        let all_combatants = self.setup_combatants(&players, &enemies);

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

    fn setup_combatants(&self, players: &Vec<IntMutCombatant>, enemies: &Vec<IntMutCombatant>) -> Vec<CombatantWithRelations> {
        let players_with_relations = map_to_combatants_with_relations(players, enemies);
        let mut enemies_with_relations = map_to_combatants_with_relations(enemies, players);

        let mut all_combatants = players_with_relations;
        all_combatants.append(&mut enemies_with_relations);
        all_combatants.sort_by_key(|x| -(D20.roll() as i16 + x.combatant.borrow().stats.initiative));
        all_combatants
    }

    fn instantiate_for_run(&self, combatants: &[CombatantConfig]) -> Vec<IntMutCombatant> {
        combatants
            .iter()
            .map(|x| Rc::new(RefCell::new(x.to_combatant())))
            .collect()
    }
}

fn map_to_combatants_with_relations(
    allies: &[IntMutCombatant],
    enemies: &[IntMutCombatant],
) -> Vec<CombatantWithRelations> {
    let allies_with_relations: Vec<_> = allies
        .to_vec()
        .into_iter()
        .map(|combatant| CombatantWithRelations {
            combatant,
            allies: allies.to_vec(),
            enemies: enemies.to_vec(),
        })
        .collect();
    allies_with_relations
}

fn run_round(combatants: &[CombatantWithRelations]) {
    for combatant in combatants {
        if combatant.combatant.borrow().is_conscious() {
            take_turn(combatant);
        }
    }
}

fn take_turn(combatant: &CombatantWithRelations) {
    combatant.combatant.borrow_mut().update_resources_on_start();
    let maybe_action = combatant.combatant.borrow().first_available_action();
    if let Some(action) = maybe_action {
        action.execute(&combatant.allies, &combatant.enemies);
        combatant
            .combatant
            .borrow_mut()
            .use_resources(action.as_ref());
    }
}

fn all_defeated(combatants: &[IntMutCombatant]) -> bool {
    combatants.iter().all(|p| !p.borrow().is_conscious())
}

fn count_survivors(combatants: &[IntMutCombatant]) -> usize {
    combatants
        .iter()
        .filter(|p| p.borrow().is_conscious())
        .count()
}
