//! Deencoding engine for a mixed UTF-8/UTF-16BE scheme
//!
//! Big Endian pendant of [`mixed816leengine`](crate::mixed816leengine); see
//! [`mixed816leengine`](crate::mixed816leengine).
use crate::engine::Engine;

pub struct Mixed816BEEngine {}

impl Engine for Mixed816BEEngine
{
    fn get_name(&self) -> String { "mixed UTF-8/UTF-16BE".to_string() }
    fn encode(&self, string: &str) -> Option<Vec<u8>>
    {
        let mut encoded = Vec::new();

        for c in string.chars()
        {
            if c.is_ascii()
            {
                encoded.push(c as u8);
            }
            else
            {
                let mut buf = [0u16; 2];
                let _ = c.encode_utf16(&mut buf);
                for i in 0..c.len_utf16()
                {
                    encoded.extend_from_slice(&buf[i].to_be_bytes());
                }
            }
        }

        Some(encoded)
    }
    fn decode(&self, bytes: &[u8]) -> String
    {
        let mut decoded = String::new();

        let mut i = 0;
        while i < bytes.len()
        {
            if bytes[i].is_ascii()
            {
                decoded.push(bytes[i] as char);
                i += 1;
            }
            else
            {
                if i + 1 == bytes.len()
                {
                    // Cannot read a second byte
                    decoded.push(char::REPLACEMENT_CHARACTER);
                    i += 1;
                    continue;
                }
                let unit1 =
                    u16::from_be_bytes(bytes[i..i+2].try_into().unwrap());
                if unit1 < 0xD800 || 0xE000 <= unit1
                {
                    // Single unit, not a surrogate
                    decoded.extend(
                        char::decode_utf16([unit1])
                        // Iterator<Result<char, DecodeUtf16Error>>
                        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER)));
                    i += 2;
                }
                else
                {
                    // Surrogate pair
                    if bytes.len() <= i + 3
                    {
                        // Cannot read a second unit
                        decoded.push(char::REPLACEMENT_CHARACTER);
                        i += 2;
                        continue;
                    }
                    let unit2 = 
                        u16::from_be_bytes(bytes[i+2..i+4].try_into().unwrap());
                    decoded.extend(
                        char::decode_utf16([unit1, unit2])
                        // Iterator<Result<char, DecodeUtf16Error>>
                        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER)));
                    i += 4;
                }
            }
        }

        decoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let engine = Mixed816BEEngine{};

        let encoded = engine.encode("Hello").unwrap();
        assert_eq!(encoded, b"Hello");

        let encoded = engine.encode("é").unwrap();
        // U+00E9
        // => 00 e9
        assert_eq!(encoded, &[0x00, 0xe9]);

        let encoded = engine.encode("😀").unwrap();
        // U+1F600
        // => 1 1111 0110 0000 0000
        // => 0000111101 1000000000
        // => 110110 0000111101 110111 1000000000
        // => U+D83D U+DE00
        // => d8 3d de 00
        assert_eq!(encoded, &[0xd8, 0x3d, 0xde, 0x00]);
    }

    #[test]
    fn decode()
    {
        let engine = Mixed816BEEngine{};

        let decoded = engine.decode(&[0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21]);
        assert_eq!(decoded, "world!");

        let decoded = engine.decode(&[0xd8, 0x3d, 0xde, 0x10]);
        // U+D83D U+DE10
        // => 1101 1000 0011 1101 1101 1110 0001 0000
        // => 110110 0000111101 110111 1000010000
        // => 0000111101 1000010000
        // => 1 1111 0110 0001 0000
        // => U+1F610
        assert_eq!(decoded, "😐");
    }
}
