use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SaveType {
    STR,
    DEX,
    CON,
    INT,
    WIS,
    CHA,
}

#[derive(Clone, Debug)]
pub struct Save {
    save_type: SaveType,
    dc: i16,
}

impl Save {
    pub fn new(save_type: SaveType, dc: i16) -> Save {
        return Self { save_type, dc };
    }

    pub fn save_type(&self) -> &SaveType {
        &self.save_type
    }

    pub fn dc(&self) -> i16 {
        self.dc
    }
}
