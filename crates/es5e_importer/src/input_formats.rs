use core::fmt;

use lib_es5e_core::combatant::{
    config::CombatantConfig, defences::save::SaveModifiers, state::ResourceCfgs,
    stats::CombatantStats,
};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct Monster {
    name: String,
    ac: Vec<ArmorClass>,
    hp: HitPoints,
    str: u32,
    dex: u32,
    con: u32,
    int: u32,
    wis: u32,
    cha: u32,
    saves: Option<Saves>,
    #[serde(rename = "trait")]
    traits: Vec<Trait>,
    action: Vec<Action>,
}

fn modifier_for_value(value: u32) -> i16 {
    ((value as i16 - 10) / 2) as i16
}

impl From<Monster> for CombatantConfig {
    fn from(monster: Monster) -> Self {
        CombatantConfig {
            stats: CombatantStats {
                max_hp: monster.hp.average,
                ac: monster.ac[0].ac,
                initiative: modifier_for_value(monster.dex),
                saves: SaveModifiers::new(
                    modifier_for_value(monster.str),
                    modifier_for_value(monster.dex),
                    modifier_for_value(monster.con),
                    modifier_for_value(monster.int),
                    modifier_for_value(monster.wis),
                    modifier_for_value(monster.cha),
                ),
            },
            actions: vec![],
            resources: ResourceCfgs::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Saves {
    #[serde(deserialize_with = "deserialise_modifier_as_string")]
    str: Option<i32>,
    #[serde(deserialize_with = "deserialise_modifier_as_string")]
    dex: Option<i32>,
    #[serde(deserialize_with = "deserialise_modifier_as_string")]
    con: Option<i32>,
    #[serde(deserialize_with = "deserialise_modifier_as_string")]
    int: Option<i32>,
    #[serde(deserialize_with = "deserialise_modifier_as_string")]
    wis: Option<i32>,
    #[serde(deserialize_with = "deserialise_modifier_as_string")]
    cha: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    source: String,
}

#[derive(Serialize, Deserialize)]
pub struct MonsterType {
    #[serde(rename = "type")]
    type_name: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ArmorClass {
    pub ac: i16,
    from: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct HitPoints {
    average: u32,
    formula: String,
}

#[derive(Serialize, Deserialize)]
pub struct Trait {
    name: String,
    entries: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    name: String,
    entries: Vec<String>,
}

// Custom deserializer for skill values
fn deserialise_modifier_as_string<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    struct SkillValueVisitor;

    impl<'de> Visitor<'de> for SkillValueVisitor {
        type Value = Option<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing a signed integer")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            v.parse::<i32>().map(|x| Some(x)).map_err(de::Error::custom)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_str(SkillValueVisitor)
}
