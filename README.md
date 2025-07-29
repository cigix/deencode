# Deencode: Reverse engineer encoding errors

My first name is Clément. Throughout my life, I've encountered my fair share of
bad printings of my name because of bad encoding management: the text is
_encoded_ (turned from an internal representation into a sequence of bytes) then
_decoded_ (turned from a sequence of bytes into an internal representation)
using different schemes. This often leads to non-ASCII characters being mangled,
replaced, or outright ignored.

For example:
```
The string "Clément"
└╴encoded as UTF-8 is 43 6C C3 A9 6D 65 6E 74
  └╴decoded as Latin-1 / Codepage 1252 is "ClÃ©ment"
```

Having this sort of visualisations is why I created this crate. You take a
number of
[_engines_](https://docs.rs/deencode/latest/deencode/engine/trait.Engine.html#implementors),
pass them to
[`deencode::deencode()`](https://docs.rs/deencode/latest/deencode/fn.deencode.html)
to get back a
[tree](https://docs.rs/deencode/latest/deencode/deencodetree/struct.DeencodeTree.html)
of possible sequences of encodings and decodings, and then work on that tree.

# Example usage

```rust
// List the engines to use.
let engines: Vec<&dyn Engine> = vec![&UTF8, &LATIN1, &MIXED816BE, &MIXED816LE, &UTF7];
// Explore the tree of possible encodings and decodings.
let mut tree = deencode("Clément", &engines, 1);
// Remove duplicate entries from the tree.
let _ = tree.deduplicate();
// Export the tree with box drawings.
println!("{}", tree);
// Export the tree as JSON.
println!("{}", serde_json::to_string(&tree).unwrap());
```

# Some additional reading
* https://mas.to/@yournameisinvalid
* https://www.kalzumeus.com/2010/06/17/falsehoods-programmers-believe-about-names/
