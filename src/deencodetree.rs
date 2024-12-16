//! The deencoding process.
//!
//! Our initial string will go through multiple encoding and decoding steps,
//! each engine (hopefully) producing a distinct output. This lead to a tree
//! structure: from the root, we get several encodings, then for each several
//! decodings, and so on.
//!
//! It is to note that we intentionally only want to end each path on a decoding
//! step, which means the depth of the tree will always be an even number.

use crate::engine::*;

use serde::Serialize;

/// An encoding step.
#[derive(Serialize)]
pub struct EncodeNode
{
    /// The name of the encoder. See [`Engine::get_name()`].
    pub name: String,
    /// The output of the encoder.
    pub output: Vec<u8>,
    /// The underlying decoding steps.
    pub decoders: Vec<DecodeNode>
}

/// A decoding step.
#[derive(Serialize)]
pub struct DecodeNode
{
    /// The name of the decoder. See [`Engine::get_name()`].
    pub name: String,
    /// The output of the decoder.
    pub output: String,
    /// The underlying encoding steps. Empty if at maximum depth.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub encoders: Vec<EncodeNode>,
    #[serde(skip)]
    is_leaf: bool
}

/// The root of the deencoding tree.
#[derive(Serialize)]
pub struct DeencodeTree
{
    /// The input string.
    pub input: String,
    /// The underlying encoding steps.
    pub encoders: Vec<EncodeNode>
}

impl EncodeNode
{
    /// <div class="warning">
    ///
    /// You probably want [`DeencodeTree::deencode()`], which calls this
    /// function in a recursive fashion.
    ///
    /// </div>
    ///
    /// Run the encoder of each engine, recording their outputs into
    /// [`EncodeNode`s](EncodeNode) and recursively calling
    /// [`DecodeNode::make_nodes()`] with a decremented depth.
    ///
    /// Note that there is always a decoding after an encoding.
    ///
    /// Since this function decrements the depth without checking it, this means
    /// the actual depth of the subtrees generated is `2 * depth - 1`, which
    /// at `depth == 0` underflows and could lead to a huge number in release
    /// mode.
    pub fn make_nodes(input: &str, engines: &[&dyn Engine],
        depth: usize)
        -> Vec<EncodeNode>
    {
        let mut results = Vec::<EncodeNode>::new();
        for &engine in engines.iter()
        {
            if let Some(output) = engine.encode(input)
            {
                let decoders =
                    DecodeNode::make_nodes(&output, engines, depth - 1);
                results.push(EncodeNode {
                    name: engine.get_name(), output, decoders
                });
            };
        }
        results
    }

    /// 1. Register the output of this node to `known_bytes`.
    /// 1. Delete any entry in `decoders` whose output is already registered to
    ///    `known_strings`.
    /// 1. Call `deduplicate()` on remaining entries.
    ///
    /// The entries are explored depth-first in the same order as the engines
    /// slice passed to `make_nodes()`.
    pub fn deduplicate(&mut self, known_strings: &mut Vec<String>,
        known_bytes: &mut Vec<Vec<u8>>)
    {
        let mut todelete: Vec<usize> = Vec::new();

        known_bytes.push(self.output.clone());

        for i in 0..self.decoders.len()
        {
            let decoder = &mut self.decoders[i];
            if known_strings.contains(&decoder.output)
            {
                todelete.push(i);
                continue;
            }
            decoder.deduplicate(known_strings, known_bytes);
            if decoder.encoders.is_empty() && !decoder.is_leaf
            {
                todelete.push(i);
            }
        }
        while let Some(i) = todelete.pop()
        {
            self.decoders.remove(i);
        }
    }
}

impl DecodeNode
{
    /// <div class="warning">
    ///
    /// You probably want [`DeencodeTree::deencode()`], which calls this
    /// function in a recursive fashion.
    ///
    /// </div>
    ///
    /// Run the decoder of each engine, recording their outputs into
    /// [`DecodeNode`s](DecodeNode) and recursively calling
    /// [`EncodeNode::make_nodes()`] with the same depth.
    ///
    /// Note that once the depth reaches 0, no more [`EncodeNode`s](EncodeNode)
    /// are created.
    ///
    /// This function does not recurse when `depth == 0`, so the actual depth of
    /// the subtrees generated is `2 * depth`.
    pub fn make_nodes(input: &[u8], engines: &[&dyn Engine],
        depth: usize)
        -> Vec<DecodeNode>
    {
        engines.iter()
            .map(|e|
                {
                    let output = e.decode(input);
                    let encoders = if 0 < depth
                    {
                        EncodeNode::make_nodes(&output, engines, depth)
                    }
                    else
                    {
                        Vec::new()
                    };
                    DecodeNode {
                        name: e.get_name(),
                        output,
                        encoders,
                        is_leaf: depth == 0
                    }
                })
        .collect()
    }

    /// 1. Register the output of this node to `known_strings`.
    /// 1. Delete any entry in `encoders` whose output is already registered to
    ///    `known_bytes`.
    /// 1. Call `deduplicate()` on remaining entries.
    ///
    /// The entries are explored depth-first in the same order as the engines
    /// slice passed to `make_nodes()`.
    pub fn deduplicate(&mut self, known_strings: &mut Vec<String>,
        known_bytes: &mut Vec<Vec<u8>>)
    {
        let mut todelete: Vec<usize> = Vec::new();

        known_strings.push(self.output.clone());

        for i in 0..self.encoders.len()
        {
            let encoder = &mut self.encoders[i];
            if known_bytes.contains(&encoder.output)
            {
                todelete.push(i);
                continue;
            }
            encoder.deduplicate(known_strings, known_bytes);
            if encoder.decoders.len() == 0
            {
                todelete.push(i);
            }
        }
        while let Some(i) = todelete.pop()
        {
            self.encoders.remove(i);
        }
    }
}

impl DeencodeTree
{
    /// Recursively call [`EncodeNode::make_nodes()`] and
    /// [`DecodeNode::make_nodes()`] to build a deencoding tree for the given
    /// input.
    ///
    /// `depth` specify the number of _encodings_ in any branch, and encodings
    /// are always followed by decodings, so the actual depth of the generated
    /// tree is `2 * depth`.
    ///
    /// The process starts with encoding, so you may not have `depth == 0`. (see
    /// [`EncodeNode::make_nodes()`]'s documentation)
    pub fn deencode(input: &str, engines: &[&dyn Engine], depth: usize)
        -> DeencodeTree
    {
        DeencodeTree {
            input: input.to_owned(),
            encoders: EncodeNode::make_nodes(input, engines, depth)
        }
    }

    /// Prune the tree by only keeping one instance of any single encoding and
    /// decoding output.
    ///
    /// Nodes are explored depth-first, in the same order as the engines slice
    /// passed to [`DeencodeTree::deencode()`], meaning earlier engines outputs
    /// are kept over later engines.
    ///
    /// Return the list of unique decodings (including the input, guaranteed to
    /// be in the first position), and the list of unique encodings.
    pub fn deduplicate(&mut self) -> (Vec<String>, Vec<Vec<u8>>)
    {
        let mut known_strings: Vec<String> = vec![self.input.clone()];
        let mut known_bytes: Vec<Vec<u8>> = Vec::new();

        let mut todelete: Vec<usize> = Vec::new();

        for i in 0..self.encoders.len()
        {
            let encoder = &mut self.encoders[i];
            if known_bytes.contains(&encoder.output)
            {
                todelete.push(i);
                continue;
            }
            encoder.deduplicate(&mut known_strings, &mut known_bytes);
        }
        while let Some(i) = todelete.pop()
        {
            self.encoders.remove(i);
        }

        (known_strings, known_bytes)
    }
}
