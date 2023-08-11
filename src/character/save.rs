use crate::utils::save::SaveType;

#[derive(Clone)]
pub struct SaveModifiers {
    str: i16,
    dex: i16,
    con: i16,
    int: i16,
    wis: i16,
    cha: i16,
}

impl SaveModifiers {
    pub fn new(str: i16, dex: i16, con: i16, int: i16, wis: i16, cha: i16) -> Self {
        SaveModifiers {
            str,
            dex,
            con,
            int,
            wis,
            cha,
        }
    }

    pub fn default() -> Self {
        SaveModifiers {
            str: 0,
            dex: 0,
            con: 0,
            int: 0,
            wis: 0,
            cha: 0,
        }
    }

    pub fn modifier(&self, save_type: &SaveType) -> i16 {
        match save_type {
            SaveType::STR => self.str,
            SaveType::DEX => self.dex,
            SaveType::CON => self.con,
            SaveType::INT => self.int,
            SaveType::WIS => self.wis,
            SaveType::CHA => self.cha,
        }
    }

    pub fn str(&self) -> i16 {
        self.str
    }

    pub fn dex(&self) -> i16 {
        self.dex
    }

    pub fn con(&self) -> i16 {
        self.con
    }

    pub fn int(&self) -> i16 {
        self.int
    }

    pub fn wis(&self) -> i16 {
        self.wis
    }

    pub fn cha(&self) -> i16 {
        self.cha
    }
}
