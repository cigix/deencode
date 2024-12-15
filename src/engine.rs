pub trait Engine
{
    fn get_name(&self) -> String;
    fn encode(&self, string: &str) -> Option<Vec<u8>>;
    fn decode(&self, bytes: &[u8]) -> String;
}
