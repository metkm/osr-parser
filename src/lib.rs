//! Simple example
//!
//! ```
//! let mut replay = osr_parser::Replay::new();
//! replay.read("./replay.osr"); // Returns result
//! ```

pub mod read;

use read::{read_byte, read_int, read_string};
use std::{fs, io::Result, mem::transmute};

#[repr(i8)]
#[derive(Debug)]
pub enum Gamemode {
    Standart = 0,
    Taiko = 1,
    CatchTheBeat = 2,
    Mania = 3,
}

impl Gamemode {
    fn from_byte(b: u8) -> Gamemode {
        unsafe { transmute(b) }
    }
}

impl Default for Gamemode {
    fn default() -> Self {
        Gamemode::Standart
    }
}

#[allow(dead_code)]
#[derive(Debug, Default,)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
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
    pub raw: Vec<u8>,
}

impl Replay {
    pub fn new() -> Self {
        Replay::default()
    }

    pub fn read(&mut self, path: &str) -> Result<Self> {
        let mut content = fs::read(path)?;

        let mut p = 0;
        let p_ref = &mut p;

        let gamemode = Gamemode::from_byte(read_byte(p_ref, &content));
        let version = read_int!(u32, p_ref, &content);
        let beatmap_md5 = read_string(p_ref, &mut content)
            .unwrap_or("Can't read beatmap md5!".to_string());
        let username = read_string(p_ref, &mut content)
            .unwrap_or("Can't read username!".to_string());
        let replay_md5 = read_string(p_ref, &mut content)
            .unwrap_or("Can't read replay md5!".to_string());
        let n300 = read_int!(u16, p_ref, &content);
        let n100 = read_int!(u16, p_ref, &content);
        let n50 = read_int!(u16, p_ref, &content);
        let geki = read_int!(u16, p_ref, &content);
        let katu = read_int!(u16, p_ref, &content);
        let misses = read_int!(u16, p_ref, &content);
        let score = read_int!(u32, p_ref, &content);
        let combo = read_int!(u16, p_ref, &content);
        let perfect = read_byte(p_ref, &content);
        let mods = read_int!(u32, p_ref, &content);
        let life_bar = read_string(p_ref, &mut content).unwrap_or("".to_string());
        let time_stamp = read_int!(usize, p_ref, &content);
        let replay_length = read_int!(u32, p_ref, &content);
        let replay_data = {
            let start = *p_ref;
            *p_ref += self.replay_length as usize;
            content[start..(replay_length as usize)].to_vec()
        };
        let score_id = read_int!(usize, p_ref, &content);

        let mod_info: Option<f64> = {
            if *p_ref != content.len() {
                Some(read_int!(usize, p_ref, &content) as f64)
            } else {
                None
            }
        };

        let raw = content;

        Ok(Self {
            gamemode,
            version,
            beatmap_md5,
            username,
            replay_md5,
            n300,
            n100,
            n50,
            geki,
            katu,
            misses,
            score,
            combo,
            perfect,
            mods,
            life_bar,
            time_stamp,
            replay_length,
            replay_data,
            score_id,
            mod_info,
            raw
        })
    }
}
