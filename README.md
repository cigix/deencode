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

Some additional reading:
* https://mas.to/@yournameisinvalid
* https://www.kalzumeus.com/2010/06/17/falsehoods-programmers-believe-about-names/
