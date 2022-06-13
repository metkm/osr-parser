use std::fs;

#[cfg(feature = "lzma")]
use std::io::Read;

use super::error::ReplayError;
use super::mode::Gamemode;
use super::read::{read_byte, read_int, read_string};

#[derive(Debug)]
pub struct Replay {
    pub gamemode: Gamemode,
    pub version: u32,
    pub beatmap_md5: Option<String>,
    pub username: Option<String>,
    pub replay_md5: Option<String>,
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
    pub life_bar: Option<String>,
    pub time_stamp: usize,
    pub replay_length: u32,
    pub replay_data: Vec<u8>,
    pub score_id: usize,
    pub mod_info: Option<f64>,
}

impl Replay {
    pub fn read(path: &str) -> Result<Self, ReplayError> {
        let mut content = fs::read(path)?;
        let mut p = 0;

        let mut _replay_length = 0;

        Ok(Self {
            gamemode     : Gamemode::from(read_byte(&mut p, &content)),
            version      : read_int!(u32, &mut p, &content),
            beatmap_md5  : read_string(&mut p, &mut content).ok(),
            username     : read_string(&mut p, &mut content).ok(),
            replay_md5   : read_string(&mut p, &mut content).ok(),
            n300         : read_int!(u16, &mut p, &content),
            n100         : read_int!(u16, &mut p, &content),
            n50          : read_int!(u16, &mut p, &content),
            geki         : read_int!(u16, &mut p, &content),
            katu         : read_int!(u16, &mut p, &content),
            misses       : read_int!(u16, &mut p, &content),
            score        : read_int!(u32, &mut p, &content),
            combo        : read_int!(u16, &mut p, &content),
            perfect      : read_byte(&mut p, &content),
            mods         : read_int!(u32, &mut p, &content),
            life_bar     : read_string(&mut p, &mut content).ok(),
            time_stamp   : read_int!(usize, &mut p, &content),
            replay_length: {
                _replay_length = read_int!(u32, &mut p, &content);
                _replay_length
            },
            replay_data: {
                let start = *&mut p;
                *&mut p = _replay_length as usize;

                content[start..(_replay_length as usize)].to_vec()
            },
            score_id: read_int!(usize, &mut p, &content),
            mod_info: {
                if *&mut p != content.len() {
                    Some(read_int!(usize, &mut p, &content) as f64)
                } else {
                    None
                }
            }
        })
    }

    #[cfg(feature = "lzma")]
    pub fn parse_lzma(&mut self) -> Result<String, lzma::Error> {
        let mut content = String::new();
        let mut reader = lzma::Reader::from(&*self.replay_data)?;
        reader.read_to_string(&mut content)?;

        Ok(content)
    }
}
