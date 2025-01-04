pub mod deencodetree;
pub mod engine;
pub mod latin1engine;
pub mod utf7engine;
pub mod utf8engine;

pub use engine::Engine;
pub use deencodetree::DeencodeTree;

/// Provided engine for Latin-1 / ISO-8859-1 / Codepage 1252.
pub static LATIN1: latin1engine::Latin1Engine = latin1engine::Latin1Engine {};
/// Provided engine for UTF-7.
pub static UTF7: utf7engine::Utf7Engine = utf7engine::Utf7Engine {};
/// Provided engine for UTF-8.
pub static UTF8: utf8engine::Utf8Engine = utf8engine::Utf8Engine {};

/// Build a [`DeencodeTree`] by successively running encodings and decodings
/// through the engines.
///
/// Alias of [`DeencodeTree::deencode()`].
///
/// `encoding_depth` specifies the number of _encoding_ steps, which are always
/// followed by a decoding step, so the actual depth of the generated tree is
/// `2 * encoding_depth`.
///
/// The process starts with encoding, so you may not have `depth == 0`. (see
/// [`EncodeNode::make_nodes()`](deencodetree::EncodeNode::make_nodes)'s
/// documentation)
///
/// The order of the engines matters for [`DeencodeTree::deduplicate()`].
pub fn deencode(input: &str, engines: &[&dyn Engine], encoding_depth: usize)
    -> DeencodeTree
{
    DeencodeTree::deencode(input, engines, encoding_depth)
}
