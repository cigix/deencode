pub trait Engine
{
    fn get_name() -> String;
    fn encode(string: &str) -> Vec<u8>;
    fn decode(bytes: &[u8]) -> String;
}
