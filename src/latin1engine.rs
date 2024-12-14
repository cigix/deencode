//! Deencoding engine for Latin-1 / Codepage 1252
//!
//! Codepage 1252 is a superset of Latin-1, so the underlying implementations
//! are actually for the former.
//!
//! Encoding is performed with
//! [`encoding_rs`](https://crates.io/crates/encoding_rs), which does not
//! support insertion of U+FFFD � REPLACEMENT CHARACTER at decoding. Decoding is
//! performed with [`mail-parser`](https://crates.io/crates/mail-parser), which
//! does not allow encoding.

use crate::engine::Engine;

use encoding_rs::*;
use mail_parser::*;

pub struct Latin1Engine {}

impl Engine for Latin1Engine
{
    fn get_name() -> String { "Latin-1 / Codepage 1252".to_string() }

    fn encode(string: &str) -> Option<Vec<u8>>
    {
        let (output, _, error) = WINDOWS_1252.encode(string);
        if error
        {
            None
        }
        else
        {
            Some(output.into_owned())
        }
    }

    fn decode(bytes: &[u8]) -> String
    {
        decoders::charsets::single_byte::decoder_cp1252(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let encoded = Latin1Engine::encode("Hello").unwrap();
        assert_eq!(encoded, &[0x48, 0x65, 0x6c, 0x6c, 0x6f]);

        let encoded = Latin1Engine::encode("é").unwrap();
        assert_eq!(encoded, &[0xe9]);

        let encoded = Latin1Engine::encode("€").unwrap();
        assert_eq!(encoded, &[0x80]);

        assert!(Latin1Engine::encode("😀").is_none());
    }

    #[test]
    fn decode()
    {
        let decoded =
            Latin1Engine::decode(&[0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21]);
        assert_eq!(decoded, "world!");

        let decoded = Latin1Engine::decode(&[0xe8]);
        assert_eq!(decoded, "è");

        let decoded = Latin1Engine::decode(&[0x81]);
        assert_eq!(decoded, "�");
    }
}
