//! Deencoding engine for UTF-7
//!
//! Encoding is performed with [`utf7_imap`](https://crates.io/crates/utf7-imap)
//! which actually implements the modified UTF-7 of RFC 3501 (§5.1.3).

use crate::engine::Engine;

use utf7_imap::{decode_utf7_imap,encode_utf7_imap};

pub struct Utf7Engine {}

impl Engine for Utf7Engine
{
    fn get_name(&self) -> String { "UTF-7".to_string() }
    fn encode(&self, string: &str) -> Option<Vec<u8>>
    {
        Some(Vec::from(encode_utf7_imap(string.to_owned()).as_bytes()))
    }
    fn decode(&self, bytes: &[u8]) -> String
    {
        decode_utf7_imap(String::from_utf8_lossy(bytes).into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let engine = Utf7Engine{};

        let encoded = engine.encode("Hello").unwrap();
        assert_eq!(encoded, b"Hello");

        let encoded = engine.encode("é").unwrap();
        // U+00E9
        // => 0000 0000 1110 1001
        // => 000000 001110 100100
        // => AOk
        assert_eq!(encoded, b"&AOk-");

        let encoded = engine.encode("€").unwrap();
        // U+20AC
        // => 0010 0000 1010 1100
        // => 001000 001010 110000
        // => IKw
        assert_eq!(encoded, b"&IKw-");

        let encoded = engine.encode("😀").unwrap();
        // U+1F600
        // => 1 1111 0110 0000 0000
        // => 0000111101 1000000000
        // => 110110 0000111101 110111 1000000000
        // => U+D83D U+DE00
        // => 1101 1000 0011 1101 1101 1110 0000 0000
        // => 110110 000011 110111 011110 000000 000000
        // => 2D3eAA
        assert_eq!(encoded, b"&2D3eAA-");
    }

    #[test]
    fn decode()
    {
        let engine = Utf7Engine{};

        let decoded = engine.decode(b"world&ACE-");
        assert_eq!(decoded, "world!");

        let decoded = engine.decode(b"&AOg-");
        // AOg
        // => 000000 001110 100000
        // => 0000 0000 1110 1000
        // => U+00E8
        assert_eq!(decoded, "è");

        let decoded = engine.decode(b"&IKQ-");
        // IKQ
        // => 001000 001010 010000
        // => 0010 0000 1010 0100
        // => U+20A4
        assert_eq!(decoded, "₤");
    }
}
