
#[cfg(test)]
pub mod tests {
    #[test]
    #[cfg(feature = "lzma")]
    fn decompress_replay() -> std::io::Result<()> {
        use std::io::Read;

        let mut replay = osr_parser::Replay::new();
        replay = replay.read("./replay.osr").unwrap();

        let mut content = String::with_capacity(replay.replay_length as usize);
        let mut reader = lzma::Reader::from(&*replay.replay_data).unwrap();

        reader.read_to_string(&mut content);
        println!("{:?}", &replay.username);

        Ok(())
    }
}
