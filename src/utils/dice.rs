use std::str::FromStr;

use rand::Rng;

use super::rollable::Rollable;

#[derive(Copy, Clone, Debug)]
pub enum Die {
    D2,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl Rollable<u32> for Die {
    fn roll(&self) -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.sides())
    }
}

impl Die {
    pub const fn sides(&self) -> u32 {
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

pub fn beats_dc<T: std::cmp::PartialOrd>(roll: T, dc: T) -> bool {
    roll >= dc
}

pub fn is_natural_20(val: u32) -> bool {
    return val == Die::D20.sides();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beats_dc() {
        // Given
        let test_cases = [
            (10, 10, true), // Equal values, should beat
            (5, 10, false), // Lower roll, should not beat
            (9, 10, false), // Lower roll, should not beat
            (-3, -3, true), // Equal values, should beat
            (20, 15, true), // Higher roll, should beat
        ];

        for &(roll, dc, expected) in &test_cases {
            // When
            let result = beats_dc(roll, dc);

            // Then
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_is_natural_20() {
        assert!(is_natural_20(20));
        for i in 0..20 {
            assert!(!is_natural_20(i))
        }
    }
}
