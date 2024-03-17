use crate::parser::combatant::{CombatantDto, SaveModifiersDto};
use lib_es5e_core::combatant::{config::CombatantConfig, defences::save::SaveModifiers};
use std::fs;
use std::path::Path;

pub fn load_combatants_from_file(file_path: &Path) -> Vec<CombatantConfig> {
    let contents =
        fs::read_to_string(file_path).expect(format!("{file_path:?} not found").as_str());
    let values: Vec<CombatantDto> = serde_yaml::from_str(contents.as_str())
        .expect(format!("Unable to parse {file_path:?}").as_str());
    let nr_combatants = values.len();
    println!("Combatants loaded from {file_path:?}: {nr_combatants}");

    values.into_iter().map(|e| e.into()).collect()
}

impl From<SaveModifiersDto> for SaveModifiers {
    fn from(saves: SaveModifiersDto) -> Self {
        SaveModifiers::new(
            saves.str, saves.dex, saves.con, saves.int, saves.wis, saves.cha,
        )
    }
}

#[cfg(test)]
mod test {
    use lib_es5e_core::combatant::config::CombatantConfig;

    use crate::{
        loader::CombatantDto, parser::actions::spell_lvl_to_resource_key, rules::SpellLvl,
    };

    // Note: the API is currently very volatile, so more detailed tests are omitted for the time being
    #[test]
    fn test_parse() {
        let yaml = "
- name: dragon
  hp: 367
  ac: 22
  init: 1
  saves:
    str: 8
    dex: 9
    con: 14
    int: 3
    wis: 9
    cha: 11
  actions:
    - !SingleAction
      attack:
        !SaveBasedAttack
        name: breath weapon
        save_dc: 22
        save_type: !DEX
        targets: 3
        damage: 15d8
        half_on_success: true
      resources:
        - !Recharge5
    - !MultiAction
      - &claws
        attack:
          !Attack
          name: claws
          atk: 15
          dmg: 2d10+8
      - *claws
      - attack:
          !Attack
          name: bite
          atk: 15
          dmg: 2d6+8";
        let combatants: Vec<CombatantDto> =
            serde_yaml::from_str(yaml).expect("unable to parse test data");
        let part: Vec<CombatantConfig> = combatants.into_iter().map(|e| e.into()).collect();
        assert_eq!(part.len(), 1);
    }

    // Note: the API is currently very volatile, so more detailed tests are omitted for the time being
    #[test]
    fn test_parse_spellcaster() {
        let yaml = "
  - name: test druid
    hp: 367
    ac: 22
    init: 1
    class_lvl: [Druid, Lvl5]
    saves:
      str: 8
      dex: 9
      con: 14
      int: 3
      wis: 9
      cha: 11
    actions:
      - !SingleAction
        attack:
            !SaveBasedAttack
            name: breath weapon
            save_dc: 22
            save_type: !DEX
            targets: 3
            damage: 15d8
            half_on_success: true
        resource_costs:
          - !SpellSlot Lvl1
        ";

        let combatants: Vec<CombatantDto> =
            serde_yaml::from_str(yaml).expect("unable to parse test data");
        println!("{combatants:?}");
        let cfg: Vec<CombatantConfig> = combatants.into_iter().map(|e| e.into()).collect();
        let resource_cfgs = &cfg.first().unwrap().resources;
        assert_eq!(
            4,
            resource_cfgs
                .get(&spell_lvl_to_resource_key(SpellLvl::Lvl1))
                .unwrap()
                .charges
        );
        assert_eq!(
            3,
            resource_cfgs
                .get(&spell_lvl_to_resource_key(SpellLvl::Lvl2))
                .unwrap()
                .charges
        );
        assert_eq!(
            2,
            resource_cfgs
                .get(&spell_lvl_to_resource_key(SpellLvl::Lvl3))
                .unwrap()
                .charges
        );
    }
}
