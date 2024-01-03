use crate::{
    attack::{attack::Attack, damage::DamageRoll, spell::Spell, weapon::WeaponType},
    character::save::SaveModifiers,
    combat::{
        action::Action,
        action_selection::{ActionSelection, StatefulAction},
        participant::Participant,
    },
    utils::{dice::Die, save::SaveType},
};

pub fn get_party() -> Vec<Participant> {
    return vec![
        init_rittersporn(),
        init_julius(),
        init_tarik(),
        init_adran(),
        init_olaf(),
    ];
}

fn init_julius() -> Participant {
    // note: debuffs badly modeled at the moment...
    let eldritch_blast = Attack::new(10, DamageRoll::new(vec![Die::D10], 5));

    let synaptic_static = Spell::new(SaveType::WIS, true, 6, vec![Die::D6; 8]);

    let ac = 21;
    let hp = 89;
    let spell_dc = 18;

    let saves = SaveModifiers::new(0, 9, 1, 1, -1, 10);

    let action_selection = ActionSelection::new(
        Action::MultiAttack(vec![eldritch_blast; 3]),
        vec![StatefulAction::new_with_charges(
            Action::SaveBasedAttack(synaptic_static.to_spell_based_attack(spell_dc)),
            3,
        )],
    );

    Participant::new(hp, ac, saves, action_selection)
}

fn init_rittersporn() -> Participant {
    // note: primarily support role, so likely badly modeled at the moment...
    let vicious_mockery = Spell::new(SaveType::DEX, true, 1, vec![Die::D4; 3]);
    let lightning_bolt = Spell::new(SaveType::DEX, true, 6, vec![Die::D6; 8]);
    let lightning_bolt_upcast = Spell::new(SaveType::DEX, true, 6, vec![Die::D6; 9]);

    let ac = 16;
    let hp = 75;
    let spell_dc = 18;

    let saves = SaveModifiers::new(0, 9, 1, 1, -1, 10);

    let action_selection = ActionSelection::new(
        Action::SaveBasedAttack(vicious_mockery.to_spell_based_attack(spell_dc)),
        vec![
            StatefulAction::new_with_charges(
                Action::SaveBasedAttack(lightning_bolt_upcast.to_spell_based_attack(spell_dc)),
                3,
            ),
            StatefulAction::new_with_charges(
                Action::SaveBasedAttack(lightning_bolt.to_spell_based_attack(spell_dc)),
                3,
            ),
        ],
    );

    Participant::new(hp, ac, saves, action_selection)
}

fn init_olaf() -> Participant {
    let great_axe = Attack::new(11, DamageRoll::new(WeaponType::Greataxe.damage_dice(), 6));
    let attack_action = Action::MultiAttack(vec![great_axe; 2]);

    let hp = 149 * 2; // doubled because of rage
    let ac = 17;
    let saves = SaveModifiers::new(10, 2, 8, 1, -1, 0);

    Participant::new(
        hp,
        ac,
        saves,
        ActionSelection::new_default_only(attack_action),
    )
}

fn init_adran() -> Participant {
    let fire_bolt = Attack::new(9, DamageRoll::new(vec![Die::D10; 3], 4));
    let disintegrate = Spell::new_with_mod(SaveType::DEX, false, 1, vec![Die::D6; 10], 40);
    let blight = Spell::new(SaveType::CON, true, 1, vec![Die::D8; 8]);
    // let fireball = Spell::new_with_mod(SaveType::DEX, true, 6, vec![Die::D6; 8], 4);
    let ac = 17; // theoretically 13, but practically always shield
    let hp = 120;
    let spell_dc = 17;

    let saves = SaveModifiers::new(0, 3, 9, 4, 2, 10);

    let action_selection = ActionSelection::new(
        Action::SingleAttack(fire_bolt),
        vec![
            StatefulAction::new_with_charges(
                Action::SaveBasedAttack(disintegrate.to_spell_based_attack(spell_dc)),
                2,
            ),
            StatefulAction::new_with_charges(
                Action::SaveBasedAttack(blight.to_spell_based_attack(spell_dc)),
                5,
            ),
        ],
    );

    Participant::new(hp, ac, saves, action_selection)
}

fn init_tarik() -> Participant {
    let dagger_shocking = Attack::new(
        10,
        DamageRoll::new(
            vec![vec![Die::D4], vec![Die::D8; 2], vec![Die::D6; 4]].concat(), // note: 7d6 sneak attack split between attacks
            5,
        ),
    );

    let dancing_blade = Attack::new(
        10 + 1,
        DamageRoll::new(
            vec![vec![Die::D8], vec![Die::D6; 3]].concat(), // note: 7d6 sneak attack split between attacks
            5 + 1,
        ),
    );

    let ac = 18;
    let hp = 107 + 50; // uncanny dodge; assumption: few attackers / attacks

    let saves = SaveModifiers::new(0, 3, 9, 4, 2, 10);

    let action_selection = ActionSelection::new_default_only(Action::MultiAttack(vec![
        dagger_shocking,
        dancing_blade,
    ]));

    Participant::new(hp, ac, saves, action_selection)
}
