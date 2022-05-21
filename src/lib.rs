//! Simple example
//!
//! ```
//! let mut replay = osr_parser::Replay::new();
//! replay.read("./replay.osr"); // Returns result
//! ```

pub mod read;

use read::{read_byte, read_int, read_string};
use std::fs;
use anyhow::{Result, bail};

#[cfg(feature = "lzma")]
use std::io::Read;

#[repr(i8)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Gamemode {
    Standart = 0,
    Taiko = 1,
    CatchTheBeat = 2,
    Mania = 3,
}

impl TryFrom<u8> for Gamemode {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Gamemode> {
        match value {
            0 => Ok(Self::Standart),
            1 => Ok(Self::Taiko),
            2 => Ok(Self::CatchTheBeat),
            3 => Ok(Self::Mania),
            _ => bail!("Failed to parse Gamemode"),
        }
    }
}

impl Default for Gamemode {
    fn default() -> Self {
        Gamemode::Standart
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Replay {
    pub gamemode: Gamemode,
    pub version: u32,
    pub beatmap_md5: String,
    pub username: String,
    pub replay_md5: String,
    pub n300: u16,
    pub n100: u16,
    pub n50: u16,
    pub geki: u16,
    pub katu: u16,
    pub misses: u16,
    pub score: u32,
    pub combo: u16,
    pub perfect: u8,
    pub mods: u32,
    pub life_bar: String,
    pub time_stamp: usize,
    pub replay_length: u32,
    pub replay_data: Vec<u8>,
    pub score_id: usize,
    pub mod_info: Option<f64>,
}

impl Replay {
    pub fn new() -> Self {
        Replay::default()
    }

    pub fn read(&mut self, path: &str) -> Result<Self> {
        let mut content = fs::read(path)?;

        let mut p = 0;
        let p_ref = &mut p;

        let mut _replay_length_temp: u32 = 0;

        Ok(Self {
            gamemode: Gamemode::try_from(read_byte(p_ref, &content))?,
            version: read_int!(u32, p_ref, &content),
            beatmap_md5: read_string(p_ref, &mut content).unwrap_or_else(|_| "Can't read beatmap md5!".to_string()),
            username: read_string(p_ref, &mut content).unwrap_or_else(|_| "Can't read username!".to_string()),
            replay_md5: read_string(p_ref, &mut content).unwrap_or_else(|_| "Can't read replay md5!".to_string()),
            n300: read_int!(u16, p_ref, &content),
            n100: read_int!(u16, p_ref, &content),
            n50: read_int!(u16, p_ref, &content),
            geki: read_int!(u16, p_ref, &content),
            katu: read_int!(u16, p_ref, &content),
            misses: read_int!(u16, p_ref, &content),
            score: read_int!(u32, p_ref, &content),
            combo: read_int!(u16, p_ref, &content),
            perfect: read_byte(p_ref, &content),
            mods: read_int!(u32, p_ref, &content),
            life_bar: read_string(p_ref, &mut content).unwrap_or_else(|_| "".to_string()),
            time_stamp: read_int!(usize, p_ref, &content),
            replay_length: {  
                _replay_length_temp = read_int!(u32, p_ref, &content);
                _replay_length_temp
            },
            replay_data: {
                let start = *p_ref;
                *p_ref = _replay_length_temp as usize;

                content[start..(_replay_length_temp as usize)].to_vec()
            },
            score_id: read_int!(usize, p_ref, &content),
            mod_info: {
                if *p_ref != content.len() {
                    Some(read_int!(usize, p_ref, &content) as f64)
                } else {
                    None
                }
            },
        })
    }

    #[cfg(feature = "lzma")]
    pub fn parse_replay_data(&mut self) -> Result<String> {
        let mut content = String::new();
        
        let mut reader = lzma::Reader::from(&*self.replay_data).expect("Can't create lzma reader");
        reader.read_to_string(&mut content)?;
        
        Ok(content)
    }
}
