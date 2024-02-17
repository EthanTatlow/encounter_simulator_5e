use crate::{
    action::effect::Effect,
    attack::{damage::Damage, save_based::SaveBasedAttack},
};

#[derive(Clone, Debug)]
pub enum NegativeEffect {
    Condition,
    Damage(Damage),
    Saveable(SaveBasedAttack),
    Multi(Vec<NegativeEffect>),
}

impl Effect for NegativeEffect {
    fn number_of_targets(&self) -> usize {
        match &self {
            Self::Saveable(atk) => atk.number_of_targets(),
            _ => todo!(),
        }
    }

    fn apply<T: crate::targeting::target::Target>(&self, target: &mut T) {
        match &self {
            Self::Saveable(atk) => atk.apply(target),
            _ => todo!(),
        }
    }
}
