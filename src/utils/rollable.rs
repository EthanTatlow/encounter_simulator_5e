use mockall::automock;

#[automock]
pub trait Rollable<T> {
    fn roll(&self) -> T;
}

pub fn roll_sum<T: std::iter::Sum, R: Rollable<T>>(dice: &[R]) -> T {
    dice.iter().map(|x| x.roll()).sum()
}

pub fn roll_max<T, R>(dice: &[R]) -> T
where
    T: std::cmp::Ord + num::Zero,
    R: Rollable<T>,
{
    let max_option = dice.iter().map(|x| x.roll()).max();
    if let Some(max) = max_option {
        max
    } else {
        T::zero()
    }
}

#[test]
fn test_roll_sum() {
    // Given
    let mut dice = vec![
        MockRollable::new(),
        MockRollable::new(),
        MockRollable::new(),
    ];
    dice[0].expect_roll().returning(|| 1);
    dice[1].expect_roll().returning(|| 10);
    dice[2].expect_roll().returning(|| 4);

    // When
    let result = roll_sum(&dice);

    // Then
    assert_eq!(result, 15);
}

#[test]
fn test_roll_max() {
    // Given
    let mut dice = vec![
        MockRollable::new(),
        MockRollable::new(),
        MockRollable::new(),
    ];
    dice[0].expect_roll().returning(|| 1);
    dice[1].expect_roll().returning(|| 10);
    dice[2].expect_roll().returning(|| 4);

    // When
    let result = roll_max(&dice);

    // Then
    assert_eq!(result, 10);
}
