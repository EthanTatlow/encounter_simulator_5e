use rand::Rng;

pub enum Die {
    D2, D4, D6, D8, D10, D12, D20
}

pub trait Rollable<T> {
    fn roll(&self) -> T;
}

impl Rollable<u8> for Die {
    fn roll(&self) -> u8 {
        return roll_die(self.get_number())
    }
}

impl Die {
    pub (crate) fn get_number(&self) -> u8 {
        match self {
            Die::D2 => 2,
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
        }
    }
}

pub fn roll_sum(dice: &[Die]) -> u16 {
    dice.iter()
        .map(|x| x.roll() as u16)
        .sum()
}

pub fn roll_max(dice: &[Die]) -> u8 {
    if dice.len() == 0 {
        return 0;
    }

    dice.iter()
        .map(|x| x.roll())
        .max()
        .unwrap()
}

pub fn roll_die(max: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=max)
}

pub fn beats_dc<T: std::cmp::PartialOrd>(roll: T, dc: T) -> bool {
    roll >= dc
}

pub fn is_natural_20(val: u8) -> bool {
    return val == 20;
}
