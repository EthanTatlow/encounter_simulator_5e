use super::dice::Die;

pub trait Meanable {
    fn mean(&self) -> f32;
}

impl Meanable for Die {
    fn mean(&self) -> f32 {
        return (self.sides() + 1) as f32 / 2.0;
    }
}

pub fn mean_sum<M: Meanable>(meanables: &[M]) -> f32 {
    meanables.iter().map(|m| m.mean()).sum()
}
