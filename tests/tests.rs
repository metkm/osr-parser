#[cfg(test)]
pub mod tests {
    #[test]
    #[cfg(feature = "lzma")]
    fn decompress_replay() {
        use std::io::Read;

        let mut replay = osr_parser::Replay::new();
        replay = replay.read("./replay.osr").unwrap();

        let mut content = String::new();
        let mut reader = lzma::Reader::from(&*replay.replay_data).unwrap();

        reader.read_to_string(&mut content);
        println!("{:?}", &content);
    }
}
