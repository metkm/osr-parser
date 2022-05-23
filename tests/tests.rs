use osr_parser::ReplayError;

#[test]
#[cfg(feature = "lzma")]
fn read_replay() -> Result<(), ReplayError> {
    let mut replay = osr_parser::Replay::read("./replay.osr")?;

    replay.parse_lzma()?;

    Ok(())
}