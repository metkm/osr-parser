#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Gamemode {
    Standart = 0,
    Taiko = 1,
    CatchTheBeat = 2,
    Mania = 3,
}

impl From<u8> for Gamemode {
    fn from(value: u8) -> Gamemode {
        match value {
            0 => Gamemode::Standart,
            1 => Gamemode::Taiko,
            2 => Gamemode::CatchTheBeat,
            3 => Gamemode::Mania,
            _ => Gamemode::Standart,
        }
    }
}
