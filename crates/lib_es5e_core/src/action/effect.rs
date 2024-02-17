use crate::targeting::target::Target;

pub trait Effect {
    fn number_of_targets(&self) -> usize;
    fn apply<T: Target>(&self, target: &mut T);
}
