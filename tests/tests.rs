use osr_parser::ReplayError;

#[test]
#[cfg(feature = "lzma")]
fn read_replay() -> Result<(), ReplayError> {
    let mut replay = osr_parser::Replay::read("./replay.osr")?;

    replay.parse_lzma()?;

    Ok(())

    // let mut replay = osr_parser::Replay::new();
    // replay = replay.read("./replay.osr");

    // use std::io::Read;

    // let mut replay = osr_parser::Replay::new();
    // replay = replay.read("./replay.osr").unwrap();

    // let mut content = String::with_capacity(replay.replay_length as usize);
    // let mut reader = lzma::Reader::from(&*replay.replay_data).unwrap();

    // reader.read_to_string(&mut content);
    // println!("{:?}", &replay.username);

    // Ok(())
}