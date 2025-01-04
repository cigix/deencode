//! Deencoding engine for a mixed UTF-8/UTF-16LE scheme
//!
//! This encoding scheme is equivalent to UTF-8 for scalars in ASCII, and
//! equivalent to UTF-16LE for other scalars.
//!
//! _"But cigix, such an encoding is extremely dumb, surely nobody is actually
//! doing that"_, you might say.
//!
//! Well, yes. I couldn't find any encoding scheme matching this one, so I had
//! to roll out my own implementation, and indeed it breaks in all sorts of 
//! ways, you can even have strings that you can encode but then not decode.
//!
//! But I once had an insurance card on which `"Clément"` had become `"Cl<some
//! CJK ideogram>ent"`, that is, it didn't only mangle the `'é'`, but also
//! somehow the following `'m'`. The presence of a CJK ideogram made me think it
//! would have interpreted 2 1-byte units into 1 2-byte UTF-16 unit, but then
//! how come the rest of the string was not mangled... The quest for what
//! happened is ultimately how I ended up making this crate, and this custom
//! encoding scheme, which does yield some funky results, including one that
//! could be what was on that insurance card (which I have since lost): encoding
//! `"Clément"` as Latin-1 and then decoding it with this scheme gives
//! `"Cl淩ent"`.
use crate::engine::Engine;

pub struct Mixed816LEEngine {}

impl Engine for Mixed816LEEngine
{
    fn get_name(&self) -> String { "mixed UTF-8/UTF-16LE".to_string() }
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
                    encoded.extend_from_slice(&buf[i].to_le_bytes());
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
                    u16::from_le_bytes(bytes[i..i+2].try_into().unwrap());
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
                        u16::from_le_bytes(bytes[i+2..i+4].try_into().unwrap());
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
        let engine = Mixed816LEEngine{};

        let encoded = engine.encode("Hello").unwrap();
        assert_eq!(encoded, b"Hello");

        let encoded = engine.encode("é").unwrap();
        // U+00E9
        // => e9 00
        assert_eq!(encoded, &[0xe9, 0x00]);

        let encoded = engine.encode("😀").unwrap();
        // U+1F600
        // => 1 1111 0110 0000 0000
        // => 0000111101 1000000000
        // => 110110 0000111101 110111 1000000000
        // => U+D83D U+DE00
        // => 3d d8 00 de
        assert_eq!(encoded, &[0x3d, 0xd8, 0x00, 0xde])
    }

    #[test]
    fn decode()
    {
        let engine = Mixed816LEEngine{};

        let decoded = engine.decode(&[0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21]);
        assert_eq!(decoded, "world!");

        let decoded = engine.decode(&[0xe8, 0x00]);
        // e8 00
        // => U+00E8
        assert_eq!(decoded, "è");

        let decoded = engine.decode(&[0xa4, 0x20]);
        // a4 20
        // => U+20A4
        assert_eq!(decoded, "₤");
    }
}
