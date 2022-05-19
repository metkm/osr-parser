#[cfg(test)]
pub mod tests {
    use std::io::Result;

    #[test]
    #[cfg(feature = "lzma")]
    fn decompress_replay() -> Result<()> {
        use std::io::Read;

        let mut replay = osr_parser::Replay::new();
        replay = replay.read("./replay.osr").unwrap();

        let mut content = String::new();
        let mut reader = lzma::Reader::from(&*replay.replay_data).unwrap();

        reader.read_to_string(&mut content)?;
        println!("{:?}", &content);

        Ok(())
    }
}
