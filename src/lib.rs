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
#[derive(Debug, Default)]
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

    pub fn read(&mut self, path: &str) -> Result<()> {
        let mut content = fs::read(path)?;

        let mut p = 0;
        let p_ref = &mut p;

        self.gamemode = Gamemode::from_byte(read_byte(p_ref, &content));
        self.version = read_int!(u32, p_ref, &content);
        self.beatmap_md5 =
            read_string(p_ref, &mut content).unwrap_or("Can't read beatmap md5!".to_string());
        self.username =
            read_string(p_ref, &mut content).unwrap_or("Can't read username!".to_string());
        self.replay_md5 =
            read_string(p_ref, &mut content).unwrap_or("Can't read replay md5!".to_string());
        self.n300 = read_int!(u16, p_ref, &content);
        self.n100 = read_int!(u16, p_ref, &content);
        self.n50 = read_int!(u16, p_ref, &content);
        self.geki = read_int!(u16, p_ref, &content);
        self.katu = read_int!(u16, p_ref, &content);
        self.misses = read_int!(u16, p_ref, &content);
        self.score = read_int!(u32, p_ref, &content);
        self.combo = read_int!(u16, p_ref, &content);
        self.perfect = read_byte(p_ref, &content);
        self.mods = read_int!(u32, p_ref, &content);
        self.life_bar = read_string(p_ref, &mut content).unwrap_or("".to_string());
        self.time_stamp = read_int!(usize, p_ref, &content);
        self.replay_length = read_int!(u32, p_ref, &content);

        self.replay_data = content[*p_ref..(self.replay_length as usize)].to_vec();
        *p_ref += self.replay_length as usize;

        self.score_id = read_int!(usize, p_ref, &content);

        if *p_ref != content.len() {
            self.mod_info = Some(read_int!(usize, p_ref, &content) as f64);
        }

        self.raw = content;

        Ok(())
    }
}
