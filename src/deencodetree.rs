use crate::engine::*;

use serde::Serialize;

#[derive(Serialize)]
pub struct EncodeNode
{
    pub output: Vec<u8>,
    pub decoders: Vec<DecodeNode>,
    pub name: String
}

#[derive(Serialize)]
pub struct DecodeNode
{
    pub output: String,
    pub encoders: Vec<EncodeNode>,
    pub name: String,
    pub is_leaf: bool
}

#[derive(Serialize)]
pub struct DeencodeTree
{
    pub input: String,
    pub encoders: Vec<EncodeNode>
}

impl EncodeNode
{
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
                    output, decoders, name: engine.get_name()
                });
            };
        }
        results
    }

    pub fn deduplicate(&mut self, known_strings: &mut Vec<String>,
        known_bytes: &mut Vec<Vec<u8>>)
    {
        let mut todelete: Vec<usize> = Vec::new();

        known_bytes.push(self.output.clone());

        for i in 0..self.decoders.len()
        {
            if known_strings.contains(&self.decoders[i].output)
            {
                todelete.push(i);
                continue;
            }
            self.decoders[i].deduplicate(known_strings, known_bytes);
            if self.decoders[i].encoders.len() == 0 && !self.decoders[i].is_leaf
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
                        output,
                        encoders,
                        name: e.get_name(),
                        is_leaf: depth == 0
                    }
                })
        .collect()
    }

    pub fn deduplicate(&mut self, known_strings: &mut Vec<String>,
        known_bytes: &mut Vec<Vec<u8>>)
    {
        let mut todelete: Vec<usize> = Vec::new();

        known_strings.push(self.output.clone());

        for i in 0..self.encoders.len()
        {
            if known_bytes.contains(&self.encoders[i].output)
            {
                todelete.push(i);
                continue;
            }
            self.encoders[i].deduplicate(known_strings, known_bytes);
            if self.encoders[i].decoders.len() == 0
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
    pub fn deencode(input: &str, engines: &[&dyn Engine], depth: usize)
        -> DeencodeTree
    {
        DeencodeTree {
            input: input.to_owned(),
            encoders: EncodeNode::make_nodes(input, engines, depth)
        }
    }

    pub fn deduplicate(&mut self) -> (Vec<String>, Vec<Vec<u8>>)
    {
        let mut known_strings: Vec<String> = vec![self.input.clone()];
        let mut known_bytes: Vec<Vec<u8>> = Vec::new();

        let mut todelete: Vec<usize> = Vec::new();

        for i in 0..self.encoders.len()
        {
            if known_bytes.contains(&self.encoders[i].output)
            {
                todelete.push(i);
                continue;
            }
            self.encoders[i].deduplicate(&mut known_strings, &mut known_bytes);
        }
        while let Some(i) = todelete.pop()
        {
            self.encoders.remove(i);
        }

        (known_strings, known_bytes)
    }
}
