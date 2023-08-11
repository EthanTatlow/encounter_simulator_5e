use super::dice::Die;

pub trait Meanable {
    fn mean(&self) -> f32;
}

impl Meanable for Die {
    fn mean(&self) -> f32 {
        return (self.get_number() + 1) as f32 / 2.0;
    }
}
