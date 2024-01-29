// TODO: Weapons should likely be externalised rather than being hard-coded

use crate::utils::{dice::Die, probability::Meanable};

#[derive(Clone)]
pub struct Weapon {
    damage_dice: Vec<Die>,
    finesse: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum WeaponType {
    Dagger,
    Shortsword,
    Longsword,
    Greatsword,
    Rapier,
    Scimitar,
    BattleAxe,
    Warhammer,
    Greataxe,
    Handaxe,
}

impl WeaponType {
    pub fn weapon(&self) -> Weapon {
        match self {
            WeaponType::Dagger => Weapon {
                damage_dice: vec![Die::D4],
                finesse: true,
            },
            WeaponType::Shortsword => Weapon {
                damage_dice: vec![Die::D6],
                finesse: true,
            },
            WeaponType::Longsword => Weapon {
                damage_dice: vec![Die::D8],
                finesse: false,
            },
            WeaponType::Greatsword => Weapon {
                damage_dice: vec![Die::D6, Die::D6],
                finesse: false,
            },
            WeaponType::Rapier => Weapon {
                damage_dice: vec![Die::D8],
                finesse: true,
            },
            WeaponType::Scimitar => Weapon {
                damage_dice: vec![Die::D6],
                finesse: true,
            },
            WeaponType::BattleAxe => Weapon {
                damage_dice: vec![Die::D8],
                finesse: false,
            },
            WeaponType::Warhammer => Weapon {
                damage_dice: vec![Die::D8],
                finesse: false,
            },
            WeaponType::Greataxe => Weapon {
                damage_dice: vec![Die::D12],
                finesse: false,
            },
            WeaponType::Handaxe => Weapon {
                damage_dice: vec![Die::D6],
                finesse: false,
            },
        }
    }
}

impl Weapon {
    pub fn is_finesse(&self) -> bool {
        self.finesse
    }

    pub fn mean_damage(&self) -> f32 {
        self.damage_dice.iter().map(|x| x.mean()).sum()
    }

    pub fn damage_dice(&self) -> &[Die] {
        self.damage_dice.as_ref()
    }
}
