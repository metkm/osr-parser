#[cfg(test)]
pub mod tests {
    #[test]
    fn test_reading() {
        let mut replay = osr_parser::Replay::new();
        replay.read("./replay.osr").unwrap();

        println!("{:?}", &replay.life_bar);
    }
}
