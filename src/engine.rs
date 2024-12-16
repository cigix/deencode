/// A deencoding engine.
pub trait Engine
{
    /// The name of the engine.
    fn get_name(&self) -> String;

    /// Encode through the engine.
    ///
    /// Failure to encode is allowed.
    fn encode(&self, string: &str) -> Option<Vec<u8>>;

    /// Decode through the engine.
    ///
    /// Failure is not accepted: any encoding charset must be covered by Unicode
    /// and therefore Rust strings.
    fn decode(&self, bytes: &[u8]) -> String;
}
