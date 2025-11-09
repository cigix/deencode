//! Deencoding engine for Codepage 1253
//!
//! Encoding is performed with
//! [`encoding_rs`](https://crates.io/crates/encoding_rs), which does not
//! support insertion of U+FFFD � REPLACEMENT CHARACTER at decoding. Decoding is
//! performed with [`mail-parser`](https://crates.io/crates/mail-parser), which
//! does not allow encoding.

use crate::engine::Engine;

use encoding_rs::*;
use mail_parser::*;

pub struct CP1253Engine {}

impl Engine for CP1253Engine
{
    fn get_name(&self) -> String { "Codepage 1253".to_string() }

    fn encode(&self, string: &str) -> Option<Vec<u8>>
    {
        let (output, _, error) = WINDOWS_1253.encode(string);
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
        decoders::charsets::single_byte::decoder_cp1253(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let engine = CP1253Engine{};

        let encoded = engine.encode("Hello").unwrap();
        assert_eq!(encoded, &[0x48, 0x65, 0x6c, 0x6c, 0x6f]);

        let encoded = engine.encode("μ").unwrap();
        assert_eq!(encoded, &[0xec]);

        assert!(engine.encode("😀").is_none());
    }

    #[test]
    fn decode()
    {
        let engine = CP1253Engine{};

        let decoded = engine.decode(&[0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21]);
        assert_eq!(decoded, "world!");

        let decoded = engine.decode(&[0xe8]);
        assert_eq!(decoded, "θ");

        let decoded = engine.decode(&[0x81]);
        assert_eq!(decoded, "�");
    }
}
