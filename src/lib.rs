fn char(i: u8) -> char {
    match i {
        00...25 => ('A' as u8 + i) as char,
        26...51 => ('a' as u8 + (i-26)) as char,
        52...61 => ('0' as u8 + (i-52)) as char,
        62 => '+',
        63 => '/',
        _  => panic!("Out of base64 bounds")
    }
}
fn num(c: u8) -> u8 {
    match c as char {
        '=' => 0, // because adding 0 and not doing anything is the same thing
        'A'...'Z' => (c - 'A' as u8),
        'a'...'z' => (c - 'a' as u8 + 26),
        '0'...'9' => (c - '0' as u8 + 52),
        '+' => 62,
        '/' => 63,
        _ => panic!("Not a base64 character")
    }
}

/// Encode input to Base64
pub fn encode(input: &[u8]) -> String {
    let mut output = String::new();

    for bytes in input.chunks(3) {
        let mut buf = 0;
        buf += (bytes.get(0).cloned().unwrap_or(0) as u32) << 8*2;
        buf += (bytes.get(1).cloned().unwrap_or(0) as u32) << 8*1;
        buf +=  bytes.get(2).cloned().unwrap_or(0) as u32;

        for i in 0..4 {
            let j = (3-i) * 6;
            let buf = ((buf >> j) & 0x3F) as u8;

            if bytes.len() >= i {
                output.push(char(buf));
            }
        }
        while output.len() % 4 != 0 {
            output.push('=');
        }
    }

    output
}
/// Decode input from Base64
pub fn decode(input: &str) -> Vec<u8> {
    let mut output = Vec::new();

    for chars in input.as_bytes().chunks(4) {
        let mut buf = 0;
        buf += (num(chars[0]) as u32) << 6*3;
        buf += (num(chars[1]) as u32) << 6*2;
        buf += (num(chars[2]) as u32) << 6*1;
        buf +=  num(chars[3]) as u32;

        if chars[1] != '=' as u8 {
            output.push(((buf >> 8*2) & 0xFF) as u8);
        }
        if chars[2] != '=' as u8 {
            output.push(((buf >> 8*1) & 0xFF) as u8);
        }
        if chars[3] != '=' as u8 {
            output.push((buf & 0xFF) as u8);
        }
    }

    output
}

#[cfg(test)]
#[test]
fn test_encode() {
    assert_eq!(encode(&*b"Man"), "TWFu"); // Wikipedia example
    assert_eq!(encode(&*b"1234"), "MTIzNA==");
    assert_eq!(encode(&[0]), "AA==");
}
#[cfg(test)]
#[test]
fn test_decode() {
    assert_eq!(decode("TWFu"), b"Man"); // Wikipedia example
    assert_eq!(decode("MTIzNA=="), b"1234");
    assert_eq!(decode("AA=="), &[0]);
}
