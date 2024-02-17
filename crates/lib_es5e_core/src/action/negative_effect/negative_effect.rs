use crate::targeting::target::Target;

pub trait NegativeEffect {
    fn number_of_targets(&self) -> usize;
    fn apply<T: Target>(&self, target: &mut T);
}
