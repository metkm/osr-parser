#[cfg(test)]
pub mod tests {
    use std::time::Instant;

    #[test]
    fn test_reading() {
        let now = Instant::now();

        let mut replay = osr_parser::Replay::new();
        replay = replay.read("./replay.osr").unwrap();

        println!("{:?} - {:?}", &replay.replay_data, now.elapsed());
    }
}
