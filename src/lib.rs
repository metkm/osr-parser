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
    gamemode: Gamemode,
    version: u32,
    beatmap_md5: String,
    username: String,
    replay_md5: String,
    n300: u16,
    n100: u16,
    n50: u16,
    geki: u16,
    katu: u16,
    misses: u16,
    score: u32,
    combo: u16,
    perfect: u8,
    mods: u32,
    life_bar: String,
    time_stamp: usize,
    replay_length: u32,
    replay_data: Vec<u8>,
    score_id: usize,
    mod_info: Option<f64>,
    raw: Vec<u8>,
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
