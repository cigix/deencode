//! # Deencode: Reverse engineer encoding errors
//!
//! The goal of this crate is to automatically explore the result of
//! successively encoding then decoding a string using different encoding
//! schemes, which usually results in some corruption of the non-ASCII
//! characters.
//!
//! ## Concepts
//!
//! * [Engines](engine/trait.Engine.html) are objects that represent an encoding
//!   scheme, and can be used to encode (String to bytes) or decode (bytes to
//!   String). A number of engines are already implemented into this crate, with
//!   static instances if you want to use them.
//! * The structure of deencoding is a
//!   [tree](deencodetree/struct.DeencodeTree.html): from an input string, every
//!   engine may give an encoding, then every engine gives a decoding of that
//!   encoding, and so on.
//!
//! > _Note_: The deencoding process is not optimised to avoid doing the same
//! > steps over and over. It is recommended to keep the depth to small numbers.
//! > Deduplication can then be applied to remove duplication in the tree.
//!
//! ## Usage
//!
//! ```rust
//! use deencode::*;
//!
//! // List the engines to use.
//! let engines: Vec<&dyn Engine> = vec![&UTF8, &LATIN1, &MIXED816BE, &MIXED816LE, &UTF7];
//! // Explore the tree of possible encodings and decodings.
//! let mut tree = deencode("Clément", &engines, 1);
//! // Remove duplicate entries from the tree.
//! let _ = tree.deduplicate();
//!
//! // Export the tree with box drawings.
//! println!("{}", tree);
//! // Export the tree as JSON.
//! println!("{}", serde_json::to_string(&tree).unwrap());
//! ```

pub mod deencodetree;
pub mod engine;
pub mod latin1engine;
pub mod latin2engine;
pub mod mixed816beengine;
pub mod mixed816leengine;
pub mod utf7engine;
pub mod utf8engine;

pub use engine::Engine;
pub use deencodetree::DeencodeTree;

/// Provided engine for Latin-1 / ISO-8859-1 / Codepage 1252.
pub static LATIN1: latin1engine::Latin1Engine = latin1engine::Latin1Engine {};
/// Provided engine for Latin-2 / ISO-8859-2 / Codepage 1250.
pub static LATIN2: latin2engine::Latin2Engine = latin2engine::Latin2Engine {};
/// Provided engine for a mixed UTF-8/UTF-16BE scheme.
pub static MIXED816BE: mixed816beengine::Mixed816BEEngine =
    mixed816beengine::Mixed816BEEngine {};
/// Provided engine for a mixed UTF-8/UTF-16LE scheme.
pub static MIXED816LE: mixed816leengine::Mixed816LEEngine =
    mixed816leengine::Mixed816LEEngine {};
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
