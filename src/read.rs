use std::string::FromUtf8Error;

pub fn read_byte(p: &mut usize, buff: &[u8]) -> u8 {
    let byte = buff[*p];
    *p += 1;
    byte
}

// 4 byte little endian value.
pub fn read_integer(p: &mut usize, buff: &[u8]) -> u32 {
    let mut val: u32 = 0;

    let mut offset = 0;
    for _ in 0..4 {
        val |= (read_byte(p, buff) as u32) << offset;
        offset += 8;
    }

    val
}

pub fn read_short(p: &mut usize, buff: &[u8]) -> u16 {
    let mut val: u16 = 0;

    let mut offset = 0;
    for _ in 0..2 {
        val |= (read_byte(p, buff) as u16) << offset;
        offset += 8;
    }

    val
}

pub fn read_long(p: &mut usize, buff: &[u8]) -> usize {
    let mut val: usize = 0;
    
    let mut offset = 0;
    let x = std::mem::size_of::<usize>();

    for _ in 0..x {
        val |= (read_byte(p, buff) as usize) << offset;
        offset += 8;
    }

    val
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
