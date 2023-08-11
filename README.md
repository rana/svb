# svb
Stream variable byte compression in Rust.

## Control header

One byte header for four 32-bit integer compressions.

Four header values in the byte.

| Compression Length | 1 byte | 2 bytes | 3 bytes | 4 bytes |
|--------------------|--------|---------|---------|---------|
| Bit value          | 00     | 01      | 10      | 11      |
| Integer value      | 0      | 1       | 2       | 3       |

In the header byte, bit values are indexed from right to left.
So, the right-most two bits are for the first compressed integer.

| Header Index    | 3  | 2  | 1  | 0  |
|-----------------|----|----|----|----|
| Some bit values | 00 | 00 | 11 | 01 |

## Development notes

Lemire blog: [Stream VByte: breaking new speed records for integer compression](https://lemire.me/blog/2017/09/27/stream-vbyte-breaking-new-speed-records-for-integer-compression/)

asXiv article: [Stream VByte: Faster Byte-Oriented Integer Compression](https://arxiv.org/abs/1709.08990)

Lemire C code: [streamvbyte](https://github.com/lemire/streamvbyte)
* Good overview of format in README.

Pierce Rust code: [stream-vbyte-rust](https://bitbucket.org/marshallpierce/stream-vbyte-rust/src/master/)

