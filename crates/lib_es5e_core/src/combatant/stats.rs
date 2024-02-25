use super::defences::save::SaveModifiers;

#[derive(Clone, Debug)]
pub struct CombatantStats {
    pub max_hp: u32,
    pub ac: i16,
    pub initiative: i16,
    pub saves: SaveModifiers,
}
