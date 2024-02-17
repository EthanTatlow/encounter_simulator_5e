use crate::{attack::damage::Damage, combatant::defences::save::SaveModifiers};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Target {
    fn is_conscious(&self) -> bool;
    fn take_damage(&mut self, damage: Damage);
    fn ac(&self) -> i16;
    fn hp(&self) -> u32;
    fn saves(&self) -> &SaveModifiers;
}
