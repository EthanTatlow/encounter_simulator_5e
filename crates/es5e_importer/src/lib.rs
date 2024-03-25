mod input_formats;
mod action;

#[cfg(test)]
mod tests {
    use lib_es5e_core::combatant::config::CombatantConfig;

    use crate::input_formats;

    const TEST_DATA: &str = r#"{
        "name": "Orc",
        "source": "MM",
        "page": 246,
        "srd": true,
        "basicRules": true,
        "otherSources": [
            {
                "source": "HotDQ"
            },
            {
                "source": "LMoP"
            },
            {
                "source": "PotA"
            },
            {
                "source": "SKT"
            },
            {
                "source": "TftYP"
            },
            {
                "source": "ToA"
            },
            {
                "source": "GoS"
            },
            {
                "source": "DIP"
            },
            {
                "source": "ERLW"
            },
            {
                "source": "RMBRE"
            },
            {
                "source": "EGW"
            },
            {
                "source": "IDRotF"
            },
            {
                "source": "CRCotN"
            },
            {
                "source": "HftT"
            },
            {
                "source": "PaBTSO"
            }
        ],
        "size": [
            "M"
        ],
        "type": {
            "type": "humanoid",
            "tags": [
                "orc"
            ]
        },
        "alignment": [
            "C",
            "E"
        ],
        "ac": [
            {
                "ac": 13,
                "from": [
                    "{@item hide armor|phb}"
                ]
            }
        ],
        "hp": {
            "average": 15,
            "formula": "2d8 + 6"
        },
        "speed": {
            "walk": 30
        },
        "str": 16,
        "dex": 12,
        "con": 16,
        "int": 7,
        "wis": 11,
        "cha": 10,
        "skill": {
            "intimidation": "+2"
        },
        "senses": [
            "darkvision 60 ft."
        ],
        "passive": 10,
        "languages": [
            "Common",
            "Orc"
        ],
        "cr": "1/2",
        "trait": [
            {
                "name": "Aggressive",
                "entries": [
                    "As a bonus action, the orc can move up to its speed toward a hostile creature that it can see."
                ]
            }
        ],
        "action": [
            {
                "name": "Greataxe",
                "entries": [
                    "{@atk mw} {@hit 5} to hit, reach 5 ft., one target. {@h}9 ({@damage 1d12 + 3}) slashing damage."
                ]
            },
            {
                "name": "Javelin",
                "entries": [
                    "{@atk mw,rw} {@hit 5} to hit, reach 5 ft. or range 30/120 ft., one target. {@h}6 ({@damage 1d6 + 3}) piercing damage."
                ]
            }
        ],
        "environment": [
            "underdark",
            "mountain",
            "grassland",
            "forest",
            "swamp",
            "hill",
            "arctic"
        ],
        "soundClip": {
            "type": "internal",
            "path": "bestiary/orc.mp3"
        },
        "altArt": [
            {
                "name": "Iceshield Orc",
                "source": "PotA"
            }
        ],
        "attachedItems": [
            "greataxe|phb",
            "javelin|phb"
        ],
        "traitTags": [
            "Aggressive"
        ],
        "senseTags": [
            "D"
        ],
        "languageTags": [
            "C",
            "O"
        ],
        "damageTags": [
            "P",
            "S"
        ],
        "miscTags": [
            "MLW",
            "MW",
            "RW",
            "THW"
        ],
        "hasToken": true,
        "hasFluff": true,
        "hasFluffImages": true
    }"#;

    #[test]
    fn it_works() {
        let monster: input_formats::Monster = serde_json::from_str(TEST_DATA).unwrap();
        println!("{:?}", CombatantConfig::from(monster));
    }
}
