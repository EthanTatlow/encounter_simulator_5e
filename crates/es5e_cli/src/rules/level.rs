use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(PartialEq, PartialOrd, Clone, Copy, Deserialize, Serialize, Debug)]
pub enum CharacterLvl {
    Lvl1,
    Lvl2,
    Lvl3,
    Lvl4,
    Lvl5,
    Lvl6,
    Lvl7,
    Lvl8,
    Lvl9,
    Lvl10,
    Lvl11,
    Lvl12,
    Lvl13,
    Lvl14,
    Lvl15,
    Lvl16,
    Lvl17,
    Lvl18,
    Lvl19,
    Lvl20,
}

#[derive(EnumIter, PartialEq, Eq, Hash, Clone, Debug)]
pub enum SpellLvl {
    Lvl1,
    Lvl2,
    Lvl3,
    Lvl4,
    Lvl5,
    Lvl6,
    Lvl7,
    Lvl8,
    Lvl9,
}
