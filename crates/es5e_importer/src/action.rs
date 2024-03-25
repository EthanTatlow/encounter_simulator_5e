use lib_es5e_core::{action::{attack::Attack, single::{Execution, SingleAction}}, attack::damage::DamageRoll, combatant::{config::ActionType, state::ResourceCosts}, utils::dice::Die};
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
pub struct Action {
    name: String,
    entries: Vec<String>,
}

impl From<Action> for ActionType {
    fn from(action: Action) -> Self {
        let entry = action.entries[0].clone();

        let re = Regex::new(r"\{@atk ([a-z],?)*\}").unwrap();
        if re.is_match(&entry) {
            println!("matched");
            ActionType::SingleAction(SingleAction { resource_cost: ResourceCosts::new(), execution: Execution::Attack(Attack::new(1, DamageRoll::new(vec![Die::D20], 6))) })
        }
        else {
            ActionType::SingleAction(SingleAction { resource_cost: ResourceCosts::new(), execution: Execution::Attack(Attack::new(1, DamageRoll::new(vec![Die::D20], 6))) })
        }
    }
}

mod tests {
    use super::Action;
    use super::ActionType;

    #[test]
    fn test_action_type() {
        let action = Action {
            name: "test".to_string(),
            entries: vec!["{@atk mw,rw} {@hit 7} to hit, reach 5 ft. and range 20/60 ft., one target. {@h}11 ({@damage 2d6 + 4}) piercing damage, or 13 ({@damage 2d8 + 4}) piercing damage if used with two hands to make a melee attack.".to_string()],
        };
        let action_type = ActionType::from(action);
    }
}
