use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::rules::level::{CharacterLvl, SpellLvl};

pub(super) enum CasterType {
    Full,
    Half,
    Warlock,
}

impl CasterType {
    pub fn spell_slots_for_char_lvl(&self, char_lvl: CharacterLvl) -> HashMap<SpellLvl, u32> {
        let spell_slot_progression = match self {
            CasterType::Full => full_caster::spell_slot_progression,
            CasterType::Half => half_caster::spell_slot_progression,
            CasterType::Warlock => todo!(),
        };
        SpellLvl::iter()
            .filter_map(|spell_lvl| {
                spell_slots_from_progression(char_lvl, spell_slot_progression(spell_lvl.clone()))
                    .and_then(|slots| Some((spell_lvl, slots)))
            })
            .collect()
    }
}

mod full_caster {
    use crate::rules::level::{CharacterLvl, SpellLvl};

    pub fn spell_slot_progression(spell_lvl: SpellLvl) -> Vec<(CharacterLvl, u32)> {
        match spell_lvl {
            SpellLvl::Lvl1 => vec![
                (CharacterLvl::Lvl1, 2),
                (CharacterLvl::Lvl2, 3),
                (CharacterLvl::Lvl3, 4),
            ],
            SpellLvl::Lvl2 => vec![(CharacterLvl::Lvl3, 2), (CharacterLvl::Lvl4, 3)],
            SpellLvl::Lvl3 => vec![(CharacterLvl::Lvl5, 2), (CharacterLvl::Lvl6, 3)],
            SpellLvl::Lvl4 => vec![
                (CharacterLvl::Lvl7, 1),
                (CharacterLvl::Lvl8, 2),
                (CharacterLvl::Lvl9, 3),
            ],
            SpellLvl::Lvl5 => vec![
                (CharacterLvl::Lvl9, 1),
                (CharacterLvl::Lvl10, 2),
                (CharacterLvl::Lvl18, 3),
            ],
            SpellLvl::Lvl6 => vec![(CharacterLvl::Lvl11, 1), (CharacterLvl::Lvl19, 2)],
            SpellLvl::Lvl7 => vec![(CharacterLvl::Lvl13, 1), (CharacterLvl::Lvl20, 2)],
            SpellLvl::Lvl8 => vec![(CharacterLvl::Lvl15, 1)],
            SpellLvl::Lvl9 => vec![(CharacterLvl::Lvl17, 1)],
        }
    }
}

mod half_caster {
    use crate::rules::level::{CharacterLvl, SpellLvl};

    pub fn spell_slot_progression(spell_lvl: SpellLvl) -> Vec<(CharacterLvl, u32)> {
        match spell_lvl {
            SpellLvl::Lvl1 => vec![
                (CharacterLvl::Lvl2, 2),
                (CharacterLvl::Lvl3, 3),
                (CharacterLvl::Lvl5, 4),
            ],
            SpellLvl::Lvl2 => vec![(CharacterLvl::Lvl5, 2), (CharacterLvl::Lvl7, 3)],
            SpellLvl::Lvl3 => vec![(CharacterLvl::Lvl9, 2), (CharacterLvl::Lvl11, 3)],
            SpellLvl::Lvl4 => vec![
                (CharacterLvl::Lvl13, 1),
                (CharacterLvl::Lvl15, 2),
                (CharacterLvl::Lvl17, 3),
            ],
            SpellLvl::Lvl5 => vec![(CharacterLvl::Lvl17, 1), (CharacterLvl::Lvl19, 2)],
            _ => vec![], // Half-casters do not have spell slots for levels 6-9 according to the given data
        }
    }
}

fn spell_slots_from_progression(
    char_lvl: CharacterLvl,
    mut progression: Vec<(CharacterLvl, u32)>,
) -> Option<u32> {
    progression.reverse();
    for (lvl, slots) in progression.into_iter() {
        if char_lvl >= lvl {
            return Some(slots);
        }
    }
    None
}
