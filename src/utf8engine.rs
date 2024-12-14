use crate::engine::Engine;

pub struct Utf8Engine {}

impl Engine for Utf8Engine
{
    fn get_name() -> String { "UTF-8".to_string() }
    fn encode(string: &str) -> Option<Vec<u8>>
    {
        Some(Vec::from(string.as_bytes()))
    }
    fn decode(bytes: &[u8]) -> String
    {
        String::from_utf8_lossy(bytes) // Cow<'_, str>
            .into_owned() // &str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let encoded = Utf8Engine::encode("Hello").unwrap();
        assert_eq!(encoded, &[0x48, 0x65, 0x6c, 0x6c, 0x6f]);

        let encoded = Utf8Engine::encode("é").unwrap();
        // U+00E9
        // => 1110 1001
        // => 000011 101001
        // => Leading2(00011) Continuation(101001)
        // => 110_00011 10_101001
        // => c3 a9
        assert_eq!(encoded, &[0xc3, 0xa9]);

        let encoded = Utf8Engine::encode("€").unwrap();
        // U+20AC
        // => 0010 0000 1010 1100
        // => 0010 000010 101100
        // => Leading3(0010) Continuation(000010) Continuation(101100)
        // => 1110_0010 10_000010 10_101100
        // => e2 82 ac
        assert_eq!(encoded, &[0xe2, 0x82, 0xac]);

        let encoded = Utf8Engine::encode("😀").unwrap();
        // U+1F600
        // => 1 1111 0110 0000 0000
        // => 011111 011000 000000
        // => Leading4(000) Continuation(011111) Continuation(011000) Continuation(000000)
        // => 11110_000 10_011111 10_011000 10_00000
        // => f0 9f 98 80
        assert_eq!(encoded, &[0xf0, 0x9f, 0x98, 0x80])
    }

    #[test]
    fn decode()
    {
        let decoded = Utf8Engine::decode(&[0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21]);
        assert_eq!(decoded, "world!");

        let decoded = Utf8Engine::decode(&[0xc3, 0xa8]);
        // c3 a8
        // => 110_00011 10_101000
        // => Leading2(00011) Continuation(101000)
        // => 11 101000
        // => 1110 1000
        // => U+00E8
        assert_eq!(decoded, "è");

        let decoded = Utf8Engine::decode(&[0xe2, 0x82, 0xa4]);
        // e2 82 a4
        // => 1110_0010 10_000010 10_100100
        // => Leading3(0010) Continuation(000010) Continuation(100100)
        // => 10 000010 100100
        // => 0010 0000 1010 0100
        // => U+20A4
        assert_eq!(decoded, "₤");
    }
}
