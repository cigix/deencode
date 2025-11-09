//! Deencoding engine for Codepage 1255
//!
//! Encoding is performed with
//! [`encoding_rs`](https://crates.io/crates/encoding_rs), which does not
//! support insertion of U+FFFD � REPLACEMENT CHARACTER at decoding. Decoding is
//! performed with [`mail-parser`](https://crates.io/crates/mail-parser), which
//! does not allow encoding.

use crate::engine::Engine;

use encoding_rs::*;
use mail_parser::*;

pub struct CP1255Engine {}

impl Engine for CP1255Engine
{
    fn get_name(&self) -> String { "ISO 8859-8 / Codepage 1255".to_string() }

    fn encode(&self, string: &str) -> Option<Vec<u8>>
    {
        let (output, _, error) = WINDOWS_1255.encode(string);
        if error
        {
            None
        }
        else
        {
            Some(output.into_owned())
        }
    }

    fn decode(&self, bytes: &[u8]) -> String
    {
        decoders::charsets::single_byte::decoder_cp1255(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let engine = CP1255Engine{};

        let encoded = engine.encode("Hello").unwrap();
        assert_eq!(encoded, &[0x48, 0x65, 0x6c, 0x6c, 0x6f]);

        let encoded = engine.encode("א").unwrap();
        assert_eq!(encoded, &[0xe0]);

        assert!(engine.encode("😀").is_none());
    }

    #[test]
    fn decode()
    {
        let engine = CP1255Engine{};

        let decoded = engine.decode(&[0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21]);
        assert_eq!(decoded, "world!");

        let decoded = engine.decode(&[0x80]);
        assert_eq!(decoded, "€");

        let decoded = engine.decode(&[0x81]);
        assert_eq!(decoded, "�");
    }
}
