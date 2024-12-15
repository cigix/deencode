pub mod deencodetree;
pub mod engine;
pub mod latin1engine;
pub mod utf8engine;

pub use engine::Engine;
pub use deencodetree::DeencodeTree;

pub static LATIN1: latin1engine::Latin1Engine = latin1engine::Latin1Engine {};
pub static UTF8: utf8engine::Utf8Engine = utf8engine::Utf8Engine {};

pub fn deencode(input: &str, engines: &[&dyn Engine], encoding_depth: usize)
    -> DeencodeTree
{
    DeencodeTree::deencode(input, engines, encoding_depth)
}
