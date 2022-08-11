Small libray to parse osu! osr files!

# Example

```rs
let mut replay = osr_parser::Replay::read("./replay.osr").expect("Can't open the .osr file!");

// replay.gamemode
// replay.version
// replay.username
// .....

// lzma feature should be enabled.
let lzma_stream = replay.parse_lzma()?
```
