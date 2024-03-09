use crate::rules::level::{CharacterLvl, SpellLvl};
use crate::rules::spell_slots::CasterType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl Class {
    fn caster_type(&self) -> Option<CasterType> {
        match self {
            Class::Warlock => Some(CasterType::Warlock),
            Class::Bard | Class::Cleric | Class::Druid | Class::Sorcerer | Class::Wizard => {
                Some(CasterType::Full)
            }
            Class::Paladin | Class::Ranger => Some(CasterType::Half),
            _ => None,
        }
    }

    fn spell_slots_for_char_lvl(&self, lvl: CharacterLvl) -> HashMap<SpellLvl, u32> {
        if let Some(caster_type) = self.caster_type() {
            return caster_type.spell_slots_for_char_lvl(lvl);
        }
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::class::Class::Wizard;
    use crate::rules::level::CharacterLvl;

    #[test]
    fn do_something() {
        let class = Wizard;
        let lvl = CharacterLvl::Lvl6;
        let slots = class.spell_slots_for_char_lvl(lvl);
        println!("{slots:?}")
    }
}
