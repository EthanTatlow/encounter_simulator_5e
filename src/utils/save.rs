pub enum SaveType {
    STR,
    DEX,
    CON,
    INT,
    WIS,
    CHA,
}

pub struct Save {
    save_type: SaveType,
    dc: u8,
}

impl Save {
    pub fn save_type(&self) -> &SaveType {
        &self.save_type
    }

    pub fn dc(&self) -> u8 {
        self.dc
    }
}
