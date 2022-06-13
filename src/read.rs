use std::string::FromUtf8Error;

macro_rules! read_int {
    ($t: ty, $p: expr, $buff: expr) => {{
        let size = std::mem::size_of::<$t>();

        let mut offset = 0;
        let mut total: $t = 0;

        for _ in 0..size {
            total |= (read_byte($p, $buff) as $t) << offset;
            offset += 8;
        }

        total
    }};
}

pub(crate) use read_int;

pub fn read_byte(p: &mut usize, buff: &[u8]) -> u8 {
    let byte = buff[*p];
    *p += 1;
    byte
}

pub fn read_uleb(p: &mut usize, buff: &[u8]) -> usize {
    let mut result = 0;
    let mut shift: u8 = 0;

    loop {
        let byte = read_byte(p, buff) as usize;
        result |= (byte & 0x7f) << shift;
        shift += 7;
        if byte & 0x80 == 0 {
            break;
        }
    }

    result
}

pub fn read_string(p: &mut usize, buff: &mut [u8]) -> Result<String, FromUtf8Error> {
    let byte = read_byte(p, buff);

    let mut string_length = 0;
    if byte == 0x0b {
        string_length = read_uleb(p, buff);
    }

    let start = *p;
    let end = *p + string_length;
    *p = end;

    String::from_utf8(buff[start..end].to_vec())
}
