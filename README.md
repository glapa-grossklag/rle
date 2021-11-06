# Run-Length Encoding

Run-length encoding (RLE) is one of the simplest and most intuitive compression
methods. In short, RLE encodes repeating strings of symbols as the length of the
run. An example illustrates this best. Let's say we have the following data:

	Oh noooooo!

RLE encodes the length of the run of `o`s:

	Oh no6!

The issue with this implementation is that `o6` is ambiguous. It could either
represent a run of `o` with length 6, or a single `o` followed by a single `6`.
To address this, we could encode the length of every run in our data, regardless
of length:

	O1h1 1n1o6!1

However, this is still not good enough. This used *more* space than the original
data did! We need a method that eliminates ambiguity but is still efficient.
This is accomplished by inserting a duplicate symbol in the encoded data,
followed by the number of additional symbols required in the run, to signal the
beginning of a run:

	Oh noo4!

This is how this program encodes and decodes data, and is known as traditional
run-length encoding.

# Usage

```
USAGE:
    rle [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decode    Decode a file.
    encode    Encode a file.
    help      Prints this message or the help of the given subcommand(s)
```
