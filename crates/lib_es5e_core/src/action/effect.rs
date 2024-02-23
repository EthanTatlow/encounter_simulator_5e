use crate::combatant::combatant::Combatant;

pub trait Effect {
    fn number_of_targets(&self) -> usize;
    fn apply(&self, target: &mut Combatant);
}
